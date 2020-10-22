extern crate env_logger;
extern crate log;

mod httpdir;

#[async_std::main]
async fn main() -> tide::Result<()> {
    env_logger::init();

    httpdir::run().await?;
    Ok(())
}
