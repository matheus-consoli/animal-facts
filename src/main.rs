mod animal;
mod config;
mod fact;

use axum::{routing, Router};
use reqwest::Client as ReqwestClient;
use tracing::debug;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{animal::init_animal_index, fact::fact};

#[derive(Clone)]
pub struct AppState {
    http_client: ReqwestClient,
}

#[tokio::main]
async fn main() -> Result<(), eyre::ErrReport> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let conf = config::config()?;
    init_animal_index(conf.sources)?;
    let addr = format!("{}:{}", conf.addr, conf.port).parse().unwrap();

    let router = Router::new()
        .route("/fact", routing::get(fact))
        .with_state(AppState {
            http_client: ReqwestClient::new(),
        });

    debug!("starting server at {addr}");
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
    Ok(())
}
