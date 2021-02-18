use redbox::server;

#[tokio::main]
async fn main() {
    let port = 63790; // Redis port is 6379
    let addr = format!("127.0.0.1:{}", port);
    if let Err(err) = server::run(&addr).await {
        panic!("Redbox failed: {}", err);
    }
}
