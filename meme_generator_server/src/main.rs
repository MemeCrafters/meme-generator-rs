use meme_generator::config::MEME_CONFIG;
use meme_generator_server::run_server;

#[tokio::main]
async fn main() {
    run_server(MEME_CONFIG.server.host, MEME_CONFIG.server.port).await;
}
