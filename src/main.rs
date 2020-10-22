extern crate env_logger;
extern crate log;


mod httpdir;
mod args;

use structopt::StructOpt;
use args::Opt;

#[async_std::main]
async fn main() -> tide::Result<()> {
    env_logger::init();

    httpdir::run(Opt::from_args()).await?;
    Ok(())
}
