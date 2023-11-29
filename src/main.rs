mod request;
mod inter;
use inter::init;

#[tokio::main]
async fn main() {
    init().await;
}