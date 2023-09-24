use pongers::run;

fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();

    smol::block_on(async {
        run().await;

        Ok(())
    })
}
