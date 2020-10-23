use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "httpdir", about = "Serve a direcotry over http")]
pub struct Opt {
    #[structopt(default_value = "./")]
    pub dir: String,

    #[structopt(short = "p", long = "port", default_value = "8888")]
    pub port: u32,
}
