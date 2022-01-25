use std::sync::Arc;

use axum::{
    extract::{
        ws::{self, WebSocket},
        Extension, WebSocketUpgrade,
    },
    response::IntoResponse,
};
use hyper::StatusCode;
use telemetry::{prelude::*, TelemetryLevel};

use super::{
    extract::LimitRequestGuard,
    routes::{State, WatchKeepalive},
};
use crate::{
    server::{qualification_check, resolver_function, watch},
    LivenessStatus, Message, QualificationCheckResultSuccess, ReadinessStatus,
    ResolverFunctionResultSuccess,
};

#[allow(clippy::unused_async)]
pub async fn liveness() -> (StatusCode, &'static str) {
    (StatusCode::OK, LivenessStatus::Ok.into())
}

#[allow(clippy::unused_async)]
pub async fn readiness() -> Result<&'static str, StatusCode> {
    Ok(ReadinessStatus::Ready.into())
}

#[allow(clippy::unused_async)]
pub async fn ws_watch(
    wsu: WebSocketUpgrade,
    Extension(watch_keepalive): Extension<Arc<WatchKeepalive>>,
) -> impl IntoResponse {
    async fn handle_socket(mut socket: WebSocket, watch_keepalive: Arc<WatchKeepalive>) {
        if let Err(err) = watch::run(watch_keepalive.clone_tx(), watch_keepalive.timeout())
            .start(&mut socket)
            .await
        {
            // An error is most likely returned when the client side terminates the websocket
            // session or if a network partition occurs, so this is our "normal" behavior
            trace!(error = ?err, "protocol finished");
        }
    }

    wsu.on_upgrade(move |socket| handle_socket(socket, watch_keepalive))
}

#[allow(clippy::unused_async)]
pub async fn ws_execute_ping(
    wsu: WebSocketUpgrade,
    limit_request_guard: LimitRequestGuard,
) -> impl IntoResponse {
    async fn handle_socket(mut socket: WebSocket, _limit_request_guard: LimitRequestGuard) {
        if let Err(ref err) = socket.send(ws::Message::Text("pong".to_string())).await {
            warn!("client disconnected; error={}", err);
        }
        if let Err(ref err) = socket.close().await {
            warn!("server failed to close websocket; error={}", err);
        }
    }

    wsu.on_upgrade(move |socket| handle_socket(socket, limit_request_guard))
}

#[allow(clippy::unused_async)]
pub async fn ws_execute_resolver(
    wsu: WebSocketUpgrade,
    Extension(state): Extension<Arc<State>>,
    Extension(telemetry_level): Extension<Arc<Box<dyn TelemetryLevel>>>,
    limit_request_guard: LimitRequestGuard,
) -> impl IntoResponse {
    async fn handle_socket(
        mut socket: WebSocket,
        state: Arc<State>,
        lang_server_debugging: bool,
        _limit_request_guard: LimitRequestGuard,
    ) {
        let proto =
            match resolver_function::execute(state.lang_server_path(), lang_server_debugging)
                .start(&mut socket)
                .await
            {
                Ok(started) => started,
                Err(err) => {
                    warn!(error = ?err, "failed to start protocol");
                    if let Err(err) =
                        fail_execute_resolver(socket, "failed to start protocol").await
                    {
                        warn!(error = ?err, "failed to fail execute resolver");
                    };
                    return;
                }
            };
        let proto = match proto.process(&mut socket).await {
            Ok(processed) => processed,
            Err(err) => {
                warn!(error = ?err, "failed to process protocol");
                if let Err(err) = fail_execute_resolver(socket, "failed to process protocol").await
                {
                    warn!(error = ?err, "failed to fail execute resolver");
                };
                return;
            }
        };
        if let Err(err) = proto.finish(socket).await {
            warn!(error = ?err, "failed to finish protocol");
        }
    }

    wsu.on_upgrade(move |socket| {
        handle_socket(
            socket,
            state,
            telemetry_level.is_debug_or_lower(),
            limit_request_guard,
        )
    })
}

async fn fail_execute_resolver(
    mut socket: WebSocket,
    message: impl Into<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let msg = Message::<ResolverFunctionResultSuccess>::fail(message).serialize_to_string()?;
    socket.send(ws::Message::Text(msg)).await?;
    socket.close().await?;
    Ok(())
}

#[allow(clippy::unused_async)]
pub async fn ws_execute_qualification(
    wsu: WebSocketUpgrade,
    Extension(state): Extension<Arc<State>>,
    Extension(telemetry_level): Extension<Arc<Box<dyn TelemetryLevel>>>,
    limit_request_guard: LimitRequestGuard,
) -> impl IntoResponse {
    async fn handle_socket(
        mut socket: WebSocket,
        state: Arc<State>,
        lang_server_debugging: bool,
        _limit_request_guard: LimitRequestGuard,
    ) {
        let proto =
            match qualification_check::execute(state.lang_server_path(), lang_server_debugging)
                .start(&mut socket)
                .await
            {
                Ok(started) => started,
                Err(err) => {
                    warn!(error = ?err, "failed to start protocol");
                    if let Err(err) =
                        fail_qualification_check(socket, "failed to start protocol").await
                    {
                        warn!(error = ?err, "failed to fail execute qualification");
                    };
                    return;
                }
            };
        let proto = match proto.process(&mut socket).await {
            Ok(processed) => processed,
            Err(err) => {
                warn!(error = ?err, "failed to process protocol");
                if let Err(err) =
                    fail_qualification_check(socket, "failed to process protocol").await
                {
                    warn!(error = ?err, "failed to fail execute qualification");
                };
                return;
            }
        };
        if let Err(err) = proto.finish(socket).await {
            warn!(error = ?err, "failed to finish protocol");
        }
    }

    wsu.on_upgrade(move |socket| {
        handle_socket(
            socket,
            state,
            telemetry_level.is_debug_or_lower(),
            limit_request_guard,
        )
    })
}

// TODO(fnichol): guess what, these fail functions can now be generic, yay!
async fn fail_qualification_check(
    mut socket: WebSocket,
    message: impl Into<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let msg = Message::<QualificationCheckResultSuccess>::fail(message).serialize_to_string()?;
    socket.send(ws::Message::Text(msg)).await?;
    socket.close().await?;
    Ok(())
}
