use crate::dir;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "httpdir", about = "Serve a direcotry over http")]
pub struct Opt {
    #[structopt(default_value = "./")]
    pub dir: String,

    #[structopt(short = "p", long = "port", default_value = "8888")]
    pub port: u32,

    #[structopt(long = "show-dotfiles")]
    pub show_dotfiles: bool,

    #[structopt(long = "first-group-by", default_value = "directories")]
    pub group_by: dir::FileGrouping,

    #[structopt(long = "sort", default_value = "atoz")]
    pub sort: dir::FileSort,
}
