extern crate env_logger;
extern crate log;

const PORT: u32 = 8888;

#[async_std::main]
async fn main() -> tide::Result<()> {
    env_logger::init();

    let mut app = tide::new();
    app.at("/").get(|_| async { Ok("Hello World") });

    app.listen(format!("http://0.0.0.0:{}", PORT)).await?;

    Ok(())
}
