extern crate env_logger;
extern crate log;

use std::future::Future;
use std::path::Path;
use std::pin::Pin;

use crate::args::Opt;
use crate::dir::{Directory, FileGrouping, FileSort, MetaFile};
use crate::page::Page;
use crate::path::CombinePath;

use async_std::fs;
use futures::stream::TryStreamExt;

const SERVE_DIR_PATH: &str = ".";
pub const SERVE_DIR_ROUTE: &str = "/httpdir";

#[derive(Debug, Clone)]
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
    app.with(directory_checker);
    app.at("/").get(serve_dir_at_route).post(upload_file);
    app.at("/*").get(serve_dir_at_route).post(upload_file);

    app.at(SERVE_DIR_ROUTE).serve_dir(SERVE_DIR_PATH)?;

    app.listen(format!("http://0.0.0.0:{}", port)).await?;

    Ok(())
}

async fn serve_dir_at_route(req: tide::Request<State>) -> tide::Result<tide::Response> {
    let cpath: &CombinePath = req.ext().unwrap();

    log::info!("URL PATH: {}", cpath.url_path());

    let State {
        root_path,
        file_grouping,
        file_sort,
        show_dotfiles,
    } = &req.state();

    let entries = fs::read_dir(cpath.dir_path()).await?;
    let dir = Directory::new(*file_grouping, *file_sort, *show_dotfiles);
    let directory: Directory = entries
        .try_fold(dir, |mut acc, e| async {
            let metafile = MetaFile::try_from(e).await?;
            acc.push(metafile);
            Ok(acc)
        })
        .await?;

    // let mut entries_html = String::new();
    // for file in directory {
    //     let (src, style) = if file.is_dir() {
    //         let sep = if cpath.is_root_url() {""} else {"/"};
    //         (format!("{}{}{}", cpath.url_path(), sep, file.name()), "directory")
    //     } else {
    //         (
    //             format!("{}{}{}", SERVE_DIR_ROUTE, cpath.url_path(), file.name()),
    //             "file",
    //         )
    //     };

    //     entries_html.push_str(&format!(
    //         "<li class=\"{}\"><a href={}>{}</a></li>",
    //         style,
    //         src,
    //         file.name()
    //     ));
    // }

    // let mut path_href = String::new();

    // let path_menu :String  = req.url().
    //     path_segments().unwrap()
    //     .map(|p| {
    //         path_href.push_str(&format!("/{}", &p));
    //         format!(r#" > <a href="{}">{}</a>"#, path_href, p)
    //     })
    //     .collect();

    // let html = INDEX_HTML.replace("{PATHMENU}", &path_menu).
    //     replace("{LIST}", &entries_html);

    let page = Page::build(req.url(), &cpath)
        .with_directory(directory)
        .build();

    let response = tide::Response::builder(tide::StatusCode::Ok)
        .content_type(tide::http::mime::HTML)
        .body(page.html())
        .build();
    Ok(response)
}

async fn upload_file(req: tide::Request<State>) -> tide::Result<tide::Response> {
    let cpath: &CombinePath = req.ext().unwrap();
    let cpath: CombinePath = cpath.clone();

    // TODO
    let file_name = req.header("file-name").unwrap();

    let file_path = format!("{}{}", cpath.dir_path(), &file_name[0]);
    let file_path = Path::new(&file_path);

    log::info!("File Upload: URL PATH: {}", req.url());
    log::info!("Save file: {:?}", file_path);

    while file_path.exists() {
        return Ok(tide::Response::new(tide::StatusCode::NotAcceptable));
    }

    let file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(file_path)
        .await?;

    let _ = async_std::io::copy(req, file).await?;

    Ok(tide::Redirect::new(cpath.url_path()).into())
}

fn directory_checker<'a>(
    mut req: tide::Request<State>,
    next: tide::Next<'a, State>,
) -> Pin<Box<dyn Future<Output = tide::Result> + Send + 'a>> {
    Box::pin(async {
        let root_path = req.state().root_path.to_string();
        let url_path = req.url().path().to_string();

        if url_path.starts_with(SERVE_DIR_ROUTE) {
            return Ok(next.run(req).await);
        }

        let combine_path = CombinePath::new(root_path, url_path);

        if combine_path.is_dir().await {
            req.set_ext(combine_path);
            return Ok(next.run(req).await);
        } else {
            return Ok(tide::Response::new(404));
        }
    })
}
