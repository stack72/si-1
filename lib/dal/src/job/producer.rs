use chrono::{DateTime, Utc};
use serde::Serialize;
use std::{collections::HashMap, convert::TryFrom};
use thiserror::Error;
use uuid::Uuid;

use super::consumer::{JobConsumer, JobConsumerCustomPayload, JobInfo};

#[derive(Error, Debug)]
pub enum JobProducerError {
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
}

pub type JobProducerResult<T> = Result<T, JobProducerError>;

pub trait JobProducer: std::fmt::Debug + Send + JobConsumer {
    fn args(&self) -> JobProducerResult<serde_json::Value>;
    fn meta(&self) -> JobProducerResult<JobMeta>;
    fn identity(&self) -> String;
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct JobMeta {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry: Option<isize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserve_for: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    #[serde(default = "HashMap::default")]
    pub custom: HashMap<String, serde_json::Value>,
}

impl TryFrom<Box<dyn JobProducer + Send + Sync>> for JobInfo {
    type Error = JobProducerError;

    fn try_from(job_producer: Box<dyn JobProducer + Send + Sync>) -> Result<Self, Self::Error> {
        let job_producer_meta = job_producer.meta()?;

        Ok(JobInfo {
            id: Uuid::new_v4().to_string(),
            kind: job_producer.type_name(),
            queue: None,
            created_at: Some(Utc::now()),
            enqueued_at: None,
            at: job_producer_meta.at,
            args: vec![
                job_producer.args()?,
                serde_json::to_value(job_producer.access_builder())?,
                serde_json::to_value(job_producer.visibility())?,
            ],
            retry: job_producer_meta.retry,
            custom: JobConsumerCustomPayload {
                extra: job_producer_meta.custom,
            },
        })
    }
}
