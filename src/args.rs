use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name="httpdir", about="Serve a direcotry over http")]
pub struct Opt {
    #[structopt(parse(from_os_str), default_value="./")]
    dir: PathBuf,

    #[structopt(short="p", long="port", default_value="8888")]
    port: u32,
}
