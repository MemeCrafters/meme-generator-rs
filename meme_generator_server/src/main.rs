use tracing_subscriber::{fmt, prelude::*, EnvFilter};

use meme_generator_server::run_server;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new("info")))
        .init();

    run_server(None, None).await;
}
