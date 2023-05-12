use async_trait::async_trait;
use axum::{response::IntoResponse, routing::get, Router};
use shuttle_crond::{CrondService, CrontabServiceJob, Msg, SendableJob, ShuttleCrond};
use shuttle_persist::{Persist, PersistInstance};
use shuttle_runtime::tracing::error;

struct MyJob {}

#[async_trait]
impl CrontabServiceJob for MyJob {
    fn schedule(&self) -> String {
        "*/2 * * * * *".to_string()
    }

    async fn run(&mut self) -> Result<(), anyhow::Error> {
        // Do stuff with access to shuttle resources.
        println!("I can do anything!");
        Ok(())
    }
}

#[shuttle_runtime::main]
async fn crond(#[Persist] persist: PersistInstance) -> ShuttleCrond {
    let job = MyJob {};

    let crond_service = CrondService::new(persist, vec![Box::new(job)]);

    let sender = crond_service.get_sender();

    let job2 = MyJob {};
    let (tx, rx) = tokio::sync::oneshot::channel::<Result<_, _>>();

    let _ = sender
        .send(Msg::NewJob(Box::new(job2), tx))
        .await
        .map_err(|_| error!("Failed to send job"));

    let _ = rx
        .await
        .map_err(|_| error!("Failed to receive confirmation"));

    Ok(crond_service)
}
