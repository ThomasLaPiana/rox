#[tokio::main]
async fn main() {
    if let Err(e) = rox::rox().await {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
