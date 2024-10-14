pub mod models;
pub mod server;
pub mod request;
#[tokio::main]
async fn main() {
    server::start().await;
}