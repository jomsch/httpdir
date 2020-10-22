extern crate env_logger;
extern crate log;

use async_std::fs;
use futures::StreamExt;

const PORT: u32 = 8888;
const SERVE_DIR_PATH: &str = ".";
const SERVE_DIR_ROUTE: &str = "/httpdir";

pub async fn run() -> tide::Result<()> {
    let mut app = tide::new();
    app.at("/").get(serve_dir_at_route);
    app.at("/*").get(serve_dir_at_route);

    app.at(SERVE_DIR_ROUTE).serve_dir(SERVE_DIR_PATH)?;

    app.listen(format!("http://0.0.0.0:{}", PORT)).await?;

    Ok(())
}

async fn serve_dir_at_route(req: tide::Request<()>) -> tide::Result<tide::Response> {
    log::info!("URL PATH: {}", &req.url());
    let url_path = &req.url().path();
    let dir_path = format!("{}{}", SERVE_DIR_PATH, url_path);

    let mut response = match fs::metadata(&dir_path).await {
        Ok(md) if md.is_dir() => tide::Response::builder(200)
            .content_type(tide::http::mime::HTML)
            .build(),
        _ => return Ok(tide::Response::new(404)),
    };

    let entries = fs::read_dir(&dir_path).await?;
    let entries: Vec<_> = entries.filter_map(|e| async { e.ok() }).collect().await;
    let mut entries_html = String::new();

    for entry in entries {
        let file_name: String = entry.file_name().to_string_lossy().into();
        let metadata = entry.metadata().await?;
        let src = if metadata.is_dir() {
            format!("/{}", file_name)
        } else {
            format!("{}{}/{}", SERVE_DIR_ROUTE, url_path, file_name)
        };

        entries_html.push_str(&format!("<li><a href={}>{}</a></li>", src, file_name));
    }

    let body = format!("<ul>{}</ul>", entries_html);

    response.set_body(body);

    Ok(response)
}
