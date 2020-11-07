extern crate env_logger;
extern crate log;

mod args;
mod dir;
mod httpdir;
mod page;
mod path;

use args::Opt;
use structopt::StructOpt;

#[async_std::main]
async fn main() -> tide::Result<()> {
    env_logger::init();

    httpdir::run(Opt::from_args()).await?;
    Ok(())
}
