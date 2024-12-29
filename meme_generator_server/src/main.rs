use meme_generator_server::run_server;

#[tokio::main]
async fn main() {
    run_server(None, None).await;
}
