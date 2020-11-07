use crate::dir::Directory;
use crate::httpdir::SERVE_DIR_ROUTE;
use crate::path::CombinePath;
use tide::http::Url;

const INDEX_HTML: &str = include_str!("../index.html");

pub struct Page {
    html: String,
}

impl Page {
    pub fn build<'a>(url: &'a Url, cpath: &'a CombinePath) -> PageBuilder<'a> {
        PageBuilder {
            url,
            cpath,
            file_upload: true,
            dir: None,
        }
    }

    pub fn html(&self) -> &str {
        &self.html
    }
}

pub struct PageBuilder<'a> {
    url: &'a Url,
    cpath: &'a CombinePath,
    file_upload: bool,
    dir: Option<Directory>,
}

impl<'a> PageBuilder<'a> {
    pub fn combine_path(mut self, cpath: &'a CombinePath) -> Self {
        self.cpath = cpath;
        self
    }

    pub fn url_path(mut self, url: &'a Url) -> Self {
        self.url = url;
        self
    }

    pub fn with_file_upload(mut self, b: bool) -> Self {
        self.file_upload = b;
        self
    }

    pub fn with_directory(mut self, dir: Directory) -> Self {
        self.dir = Some(dir);
        self
    }

    fn directory_to_html(&mut self) -> String {
        match self.dir.take() {
            Some(dir) => {
                return dir
                    .into_iter()
                    .map(|file| {
                        let (src, class) = if file.is_dir() {
                            let sep = if self.cpath.is_root_url() { "" } else { "/" };
                            (
                                format!("{}{}{}", self.cpath.url_path(), sep, file.name()),
                                "directory",
                            )
                        } else {
                            (
                                format!(
                                    "{}{}{}",
                                    SERVE_DIR_ROUTE,
                                    self.cpath.url_path(),
                                    file.name()
                                ),
                                "file",
                            )
                        };

                        format!(
                            "<li class=\"{}\"><a href={}>{}</a></li>",
                            class,
                            src,
                            file.name()
                        )
                    })
                    .collect::<String>();
            }
            None => return "".to_string(),
        }
    }

    fn path_menu(&mut self) -> String {
        let mut html = String::from(r#"<a href="/">/</a>"#);
        let mut incremental_path = String::new();

        for url_seg in self.url.path_segments().unwrap() {
            if url_seg == "" {
                continue
            }
            incremental_path.push_str(&format!("/{}", &url_seg));
            html.push_str(&format!(r#" > <a href="{}">{}</a>"#, &incremental_path, &url_seg));
        }

        html
    }

    pub fn build(&mut self) -> Page {
        let dir_html = self.directory_to_html();

        let html = INDEX_HTML
            .replace("{LIST}", &dir_html)
            .replace("{PATHMENU}", &self.path_menu());

        Page { html }
    }
}
