use pongers::run;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    run().await;
}
