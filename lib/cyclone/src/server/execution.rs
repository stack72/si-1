use std::{io, marker::PhantomData, path::PathBuf, process::Stdio, time::Duration};

use axum::extract::ws::WebSocket;
use bytes_lines_codec::BytesLinesCodec;
use futures::{SinkExt, StreamExt, TryStreamExt};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;
use telemetry::prelude::*;
use thiserror::Error;
use tokio::{
    process::{Child, ChildStdin, ChildStdout, Command},
    time,
};
use tokio_serde::{formats::SymmetricalJson, Deserializer, Framed, SymmetricallyFramed};
use tokio_util::codec::{Decoder, FramedRead, FramedWrite};

use crate::{
    process::{self, ShutdownError},
    server::WebSocketMessage,
    FunctionResult, FunctionResultFailure, FunctionResultFailureError, Message, OutputStream,
};

const TX_TIMEOUT_SECS: Duration = Duration::from_secs(5);

pub fn execute<Request, Success>(
    lang_server_path: impl Into<PathBuf>,
    lang_server_debugging: bool,
    command: String,
) -> Execution<Request, Success> {
    Execution {
        lang_server_path: lang_server_path.into(),
        lang_server_debugging,
        command,
        request_marker: PhantomData,
        success_marker: PhantomData,
    }
}

#[derive(Debug, Error)]
pub enum ExecutionError {
    #[error("failed to consume the {0} stream for the child process")]
    ChildIO(&'static str),
    #[error("failed to receive child process message")]
    ChildRecvIO(#[source] io::Error),
    #[error("failed to send child process message")]
    ChildSendIO(#[source] io::Error),
    #[error("failed to spawn child process; program={0}")]
    ChildSpawn(#[source] io::Error, PathBuf),
    #[error(transparent)]
    ChildShutdown(#[from] ShutdownError),
    #[error("failed to deserialize json message")]
    JSONDeserialize(#[source] serde_json::Error),
    #[error("failed to serialize json message")]
    JSONSerialize(#[source] serde_json::Error),
    #[error("send timeout")]
    SendTimeout(#[source] tokio::time::error::Elapsed),
    #[error("failed to close websocket")]
    WSClose(#[source] axum::Error),
    #[error("failed to receive websocket message--stream is closed")]
    WSRecvClosed,
    #[error("failed to receive websocket message")]
    WSRecvIO(#[source] axum::Error),
    #[error("failed to send websocket message")]
    WSSendIO(#[source] axum::Error),
    #[error("unexpected websocket message type: {0:?}")]
    UnexpectedMessageType(WebSocketMessage),
}

type Result<T> = std::result::Result<T, ExecutionError>;

#[derive(Debug)]
pub struct Execution<Request, Success> {
    lang_server_path: PathBuf,
    lang_server_debugging: bool,
    command: String,
    request_marker: PhantomData<Request>,
    success_marker: PhantomData<Success>,
}

impl<Request, Success> Execution<Request, Success>
where
    Request: Serialize + DeserializeOwned + std::marker::Unpin,
    Success: Serialize + DeserializeOwned,
{
    pub async fn start(self, ws: &mut WebSocket) -> Result<ExecutionStarted<Success>> {
        Self::ws_send_start(ws).await?;
        let request = Self::read_request(ws).await?;

        let mut command = Command::new(&self.lang_server_path);
        command
            .arg(&self.command)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped());
        if self.lang_server_debugging {
            command.env("DEBUG", "*").env("DEBUG_DEPTH", "5");
        }
        debug!(cmd = ?command, "spawning child process");
        let mut child = command
            .spawn()
            .map_err(|err| ExecutionError::ChildSpawn(err, self.lang_server_path.clone()))?;

        let stdin = child.stdin.take().ok_or(ExecutionError::ChildIO("stdin"))?;
        Self::child_send_function_request(stdin, request).await?;

        let stdout = {
            let stdout = child
                .stdout
                .take()
                .ok_or(ExecutionError::ChildIO("stdout"))?;
            let codec = FramedRead::new(stdout, BytesLinesCodec::new());
            SymmetricallyFramed::new(codec, SymmetricalJson::default())
        };

        Ok(ExecutionStarted { child, stdout })
    }

    async fn read_request(ws: &mut WebSocket) -> Result<Request> {
        let request = match ws.next().await {
            Some(Ok(WebSocketMessage::Text(json_str))) => {
                serde_json::from_str(&json_str).map_err(ExecutionError::JSONDeserialize)?
            }
            Some(Ok(unexpected)) => return Err(ExecutionError::UnexpectedMessageType(unexpected)),
            Some(Err(err)) => return Err(ExecutionError::WSRecvIO(err)),
            None => return Err(ExecutionError::WSRecvClosed),
        };
        Ok(request)
    }

    async fn ws_send_start(ws: &mut WebSocket) -> Result<()> {
        let msg = Message::<Success>::Start
            .serialize_to_string()
            .map_err(ExecutionError::JSONSerialize)?;

        time::timeout(TX_TIMEOUT_SECS, ws.send(WebSocketMessage::Text(msg)))
            .await
            .map_err(ExecutionError::SendTimeout)?
            .map_err(ExecutionError::WSSendIO)?;
        Ok(())
    }

    async fn child_send_function_request(stdin: ChildStdin, request: Request) -> Result<()> {
        let codec = FramedWrite::new(stdin, BytesLinesCodec::new());
        let mut stdin = SymmetricallyFramed::new(codec, SymmetricalJson::default());

        time::timeout(TX_TIMEOUT_SECS, stdin.send(request))
            .await
            .map_err(ExecutionError::SendTimeout)?
            .map_err(ExecutionError::ChildSendIO)?;
        time::timeout(TX_TIMEOUT_SECS, stdin.close())
            .await
            .map_err(ExecutionError::SendTimeout)?
            .map_err(ExecutionError::ChildSendIO)?;
        Ok(())
    }
}

type SiFramedRead = FramedRead<ChildStdout, BytesLinesCodec>;
type SiFramed<S> = Framed<SiFramedRead, S, S, SymmetricalJson<S>>;
type SiMessage<S> = LangServerMessage<S>;
type SiDecoderError = <BytesLinesCodec as Decoder>::Error;
type SiJsonError<S> = <SymmetricalJson<SiMessage<S>> as Deserializer<SiMessage<S>>>::Error;

#[derive(Debug)]
pub struct ExecutionStarted<Success> {
    child: Child,
    stdout: SiFramed<SiMessage<Success>>,
}

impl<Success> ExecutionStarted<Success>
where
    Success: Serialize + std::marker::Unpin,
    SymmetricalJson<SiMessage<Success>>: Deserializer<SiMessage<Success>>,
    SiDecoderError: From<SiJsonError<Success>>,
{
    pub async fn process(self, ws: &mut WebSocket) -> Result<ExecutionClosing<Success>> {
        let mut stream = self
            .stdout
            .map(|ls_result| match ls_result {
                Ok(ls_msg) => match ls_msg {
                    LangServerMessage::Output(output) => Ok(Message::OutputStream(output.into())),
                    LangServerMessage::Result(result) => Ok(Message::Result(result.into())),
                },
                Err(err) => Err(ExecutionError::ChildRecvIO(err)),
            })
            .map(|msg_result: Result<_>| match msg_result {
                Ok(msg) => match msg
                    .serialize_to_string()
                    .map_err(ExecutionError::JSONSerialize)
                {
                    Ok(json_str) => Ok(WebSocketMessage::Text(json_str)),
                    Err(err) => Err(err),
                },
                Err(err) => Err(err),
            });

        while let Some(msg) = stream.try_next().await? {
            ws.send(msg).await.map_err(ExecutionError::WSSendIO)?;
        }

        Ok(ExecutionClosing {
            child: self.child,
            success_marker: PhantomData,
        })
    }
}

#[derive(Debug)]
pub struct ExecutionClosing<Success> {
    child: Child,
    success_marker: PhantomData<Success>,
}

impl<Success> ExecutionClosing<Success>
where
    Success: Serialize + DeserializeOwned,
{
    pub async fn finish(mut self, mut ws: WebSocket) -> Result<()> {
        let finished = Self::ws_send_finish(&mut ws).await;
        let closed = Self::ws_close(ws).await;
        let shutdown =
            process::child_shutdown(&mut self.child, Some(process::Signal::SIGTERM), None)
                .await
                .map_err(Into::into);
        drop(self.child);

        match (finished, closed, shutdown) {
            // Everything succeeds, great!
            (Ok(_), Ok(_), Ok(_)) => Ok(()),

            // One of the steps failed, return its error
            (Ok(_), Ok(_), Err(err)) | (Ok(_), Err(err), Ok(_)) | (Err(err), Ok(_), Ok(_)) => {
                Err(err)
            }

            // 2/3 steps errored so warn about the lower priority error and return the highest
            // priority
            (Ok(_), Err(err), Err(shutdown)) => {
                warn!(error = ?shutdown, "failed to shutdown child cleanly");
                Err(err)
            }
            (Err(err), Ok(_), Err(shutdown)) => {
                warn!(error = ?shutdown, "failed to shutdown child cleanly");
                Err(err)
            }
            (Err(err), Err(closed), Ok(_)) => {
                warn!(error = ?closed, "failed to cleanly close websocket");
                Err(err)
            }

            // All steps failed so warn about the lower priorities and return the highest priority
            (Err(err), Err(closed), Err(shutdown)) => {
                warn!(error = ?shutdown, "failed to shutdown child cleanly");
                warn!(error = ?closed, "failed to cleanly close websocket");
                Err(err)
            }
        }
    }

    async fn ws_send_finish(ws: &mut WebSocket) -> Result<()> {
        let msg = Message::<Success>::Finish
            .serialize_to_string()
            .map_err(ExecutionError::JSONSerialize)?;
        time::timeout(TX_TIMEOUT_SECS, ws.send(WebSocketMessage::Text(msg)))
            .await
            .map_err(ExecutionError::SendTimeout)?
            .map_err(ExecutionError::WSSendIO)?;

        Ok(())
    }

    async fn ws_close(ws: WebSocket) -> Result<()> {
        ws.close().await.map_err(ExecutionError::WSClose)
    }
}

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "protocol", rename_all = "camelCase")]
pub enum LangServerMessage<Success> {
    Output(LangServerOutput),
    Result(LangServerResult<Success>),
}

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LangServerOutput {
    execution_id: String,
    stream: String,
    level: String,
    group: Option<String>,
    message: String,
    data: Option<Value>,
}

impl From<LangServerOutput> for OutputStream {
    fn from(value: LangServerOutput) -> Self {
        Self {
            execution_id: value.execution_id,
            stream: value.stream,
            level: value.level,
            group: value.group,
            data: value.data,
            message: value.message,
            timestamp: crate::timestamp(),
        }
    }
}

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "status", rename_all = "camelCase")]
pub enum LangServerResult<Success> {
    Success(Success),
    Failure(LangServerFailure),
}

impl<Success> From<LangServerResult<Success>> for FunctionResult<Success> {
    fn from(value: LangServerResult<Success>) -> Self {
        match value {
            LangServerResult::Success(success) => Self::Success(success),
            LangServerResult::Failure(failure) => Self::Failure(FunctionResultFailure {
                execution_id: failure.execution_id,
                error: FunctionResultFailureError {
                    kind: failure.error.kind,
                    message: failure.error.message,
                },
                timestamp: crate::timestamp(),
            }),
        }
    }
}

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LangServerFailure {
    #[serde(default)]
    execution_id: String,
    error: LangServerFailureError,
}

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
struct LangServerFailureError {
    kind: String,
    message: String,
}