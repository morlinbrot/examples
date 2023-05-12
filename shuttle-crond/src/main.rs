use async_trait::async_trait;
use axum::{response::IntoResponse, routing::get, Router};
use shuttle_axum::ShuttleAxum;

use shuttle_crond::{builder::*, *};
use shuttle_runtime::tracing::debug;

struct MyJob {}

#[async_trait]
impl Job for MyJob {
    fn schedule(&self) -> String {
        "*/2 * * * * *".to_string()
    }

    async fn run(&mut self) -> Result<(), anyhow::Error> {
        // Do stuff with access to shuttle resources.
        println!("I can do anything!");
        Ok(())
    }
}

async fn hello() -> impl IntoResponse {
    "Hello, crond!".to_string()
}

#[shuttle_runtime::main]
async fn axum_with_crond(#[Crond] crond: CrondInstance) -> ShuttleAxum {
    let router = axum::Router::new().route("/", get(hello));

    let my_job = MyJob {};

    debug!("Adding job...");
    crond.add_job(my_job).await;
    debug!("Job added");

    Ok(router.into())
}
