use std::str::FromStr;

use async_trait::async_trait;
use chrono::Utc;
use cron::Schedule;
use serde::{Deserialize, Serialize};
use shuttle_runtime::tracing::debug;
use tokio::{task::JoinHandle, time::sleep};

pub mod builder;

#[async_trait]
pub trait Job {
    fn schedule(&self) -> String;

    async fn run(&mut self) -> Result<(), anyhow::Error>;
}

// Crond Resource
#[derive(Clone, Serialize, Deserialize)]
pub struct CrondInstance {}

impl CrondInstance {
    pub async fn add_job(&self, job: impl Job + Send + 'static) -> JoinHandle<()> {
        debug!("Spawning new job");
        let hdl = tokio::spawn(async move {
            CrondInstance::run_job(job).await;
        });

        hdl
    }

    async fn run_job(mut job: impl Job) {
        debug!("Running job");

        let schedule = Schedule::from_str(&job.schedule()).expect("Failed to parse schedule");

        while let Some(next_run) = schedule.upcoming(Utc).next() {
            let next_run_in = next_run
                .signed_duration_since(chrono::offset::Utc::now())
                .to_std()
                .unwrap();

            sleep(next_run_in).await;

            let _res = job.run().await;
        }
    }
}
