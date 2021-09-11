use zeiver::Zeiver;
#[tokio::main]
async fn main() {
    Zeiver::crawl().await;
}

