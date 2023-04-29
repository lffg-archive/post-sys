use std::net::SocketAddr;

use axum::{routing::get, Router};
use once_cell::sync::Lazy;
use serde::Deserialize;
use sqlx::postgres::PgPoolOptions;
use tracing::info;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    setup_tracing();

    let _pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&CONFIG.database_url)
        .await
        .unwrap();

    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    let addr = SocketAddr::from(([0, 0, 0, 0], CONFIG.port));
    info!("listening on {addr} :)");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

/// Sets up tracing subscriber.
fn setup_tracing() {
    use tracing_subscriber::{
        fmt::{format::FmtSpan, layer},
        layer::SubscriberExt,
        util::SubscriberInitExt,
        EnvFilter,
    };

    let filter_layer = EnvFilter::try_from_default_env().unwrap_or("warn".into());
    let fmt_layer = layer().with_span_events(FmtSpan::NEW | FmtSpan::CLOSE);

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .init();
}

#[derive(Deserialize, Debug)]
struct Config {
    port: u16,
    database_url: String,
}

static CONFIG: Lazy<Config> = Lazy::new(|| envy::from_env().unwrap());
