extern crate env_logger;
extern crate log;

use async_std::fs;
use async_std::prelude::*;

const PORT: u32 = 8888;
const SERVE_DIR_PATH: &str = "./";
const SERVE_DIR_ROUTE: &str = "/httpdir";

#[async_std::main]
async fn main() -> tide::Result<()> {
    env_logger::init();

    let mut app = tide::new();
    app.at("/").get(serve_dir_at_route);
    app.at("/*").get(serve_dir_at_route);

    app.at(SERVE_DIR_ROUTE).serve_dir(SERVE_DIR_PATH)?;

    app.listen(format!("http://0.0.0.0:{}", PORT)).await?;

    Ok(())
}

async fn serve_dir_at_route(req: tide::Request<()>) -> tide::Result<tide::Response> {
    let url_path = &req.url().path()[1..];
    let file_path = format!("{}{}", SERVE_DIR_PATH, url_path);
    let file_path_metadata = fs::metadata(&file_path).await?;
    if !file_path_metadata.is_dir() {
        return Ok(tide::Response::builder(404).build());
    }

    let response = tide::Response::builder(200)
        .body(file_path)
        .content_type(tide::http::mime::HTML)
        .build();

    Ok(response)
}
