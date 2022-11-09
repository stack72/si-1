use async_trait::async_trait;

use si_data_faktory::Job;
use si_data_nats::NatsClient;
use telemetry::tracing::error;

use super::{JobQueueProcessor, JobQueueProcessorResult};
use crate::{job::{producer::JobProducer, queue::JobQueue}, DalContext};

#[derive(Clone, Debug)]
pub struct NatsProcessor {
    nats_conn: NatsClient,
    queue: JobQueue,
}

impl NatsProcessor {
    pub fn new(nats_conn: NatsClient) -> Self {
        Self {
            nats_conn,
            queue: JobQueue::new(),
        }
    }

    async fn push_all_jobs(&self) -> JobQueueProcessorResult<()> {
        while let Some(job_part) = self.queue.fetch_job().await {
            let job: Job = job_part.try_into()?;
            let msg = serde_json::to_vec(&job)?;
            if let Err(err) = self.nats_conn.publish("pinga.job", msg).await {
                error!("Nats publish failed, some jobs will be dropped");
                return Err(err.into());
            }
        }
        Ok(())
    }
}

#[async_trait]
impl JobQueueProcessor for NatsProcessor {
    async fn enqueue_job(&self, job: Box<dyn JobProducer + Send + Sync>, ctx: &DalContext) {
        self.queue.enqueue_job(job).await
    }

    async fn process_queue(&self) -> JobQueueProcessorResult<()> {
        let processor = self.clone();
        tokio::spawn(async move {
            if let Err(err) = processor.push_all_jobs().await {
                error!("Unable to push jobs to nats: {err}");
            }
        });
        Ok(())
    }
}

