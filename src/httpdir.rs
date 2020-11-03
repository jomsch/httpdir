extern crate env_logger;
extern crate log;

use crate::args::Opt;
use crate::dir::{Directory, FileGrouping, FileSort, MetaFile};

use async_std::fs;
use futures::stream::TryStreamExt;

const SERVE_DIR_PATH: &str = ".";
const SERVE_DIR_ROUTE: &str = "/httpdir";

#[derive(Clone)]
struct State {
    root_path: String,
    show_dotfiles: bool,
    file_sort: FileSort,
    file_grouping: FileGrouping,
}

pub async fn run(opt: Opt) -> tide::Result<()> {
    log::info!("Serving dir {}", &opt.dir);

    let Opt {
        port,
        dir,
        show_dotfiles,
        sort,
        group_by,
    } = opt;
    let state = State {
        root_path: dir,
        file_sort: sort,
        file_grouping: group_by,
        show_dotfiles,
    };

    let mut app = tide::with_state(state);
    app.at("/").get(serve_dir_at_route);
    app.at("/*").get(serve_dir_at_route);

    app.at(SERVE_DIR_ROUTE).serve_dir(SERVE_DIR_PATH)?;

    app.listen(format!("http://0.0.0.0:{}", port)).await?;

    Ok(())
}

async fn serve_dir_at_route(req: tide::Request<State>) -> tide::Result<tide::Response> {
    log::info!("URL PATH: {}", &req.url());
    let State {
        root_path,
        file_grouping,
        file_sort,
        show_dotfiles,
    } = &req.state();
    let root_path = &req.state().root_path;

    let url_path = &req.url().path();
    let dir_path = format!("{}{}", root_path, url_path);

    let mut response = match fs::metadata(&dir_path).await {
        Ok(md) if md.is_dir() => tide::Response::builder(200)
            .content_type(tide::http::mime::HTML)
            .build(),
        _ => return Ok(tide::Response::new(404)),
    };

    let entries = fs::read_dir(&dir_path).await?;
    let dir = Directory::new(*file_grouping, *file_sort, *show_dotfiles);
    let directory: Directory = entries
        .try_fold(dir, |mut acc, e| async {
            let metafile = MetaFile::try_from(e).await?;
            acc.push(metafile);
            Ok(acc)
        })
        .await?;

    let mut entries_html = String::new();
    for file in directory {
        let (src, style) = if file.is_dir() {
            let sep = if url_path == &"/" { "" } else { "/" };
            (format!("{}{}{}", url_path, sep, file.name()), "directory")
        } else {
            (
                format!("{}{}/{}", SERVE_DIR_ROUTE, url_path, file.name()),
                "file",
            )
        };

        entries_html.push_str(&format!(
            "<li class=\"{}\"><a href={}>{}</a></li>",
            style,
            src,
            file.name()
        ));
    }

    let html = format!("<html><body><ul>{}</ul></body></html>", entries_html);

    response.set_body(html);
    Ok(response)
}
