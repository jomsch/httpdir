use async_std::fs;

#[derive(Debug, Clone)]
pub struct CombinePath {
    root_path: String,
    url_path: String,
    dir_path: String,
}

impl CombinePath {
    pub fn new(root_path: String, url_path: String) -> Self {
        let mut dir_path = format!("{}{}", &root_path, &url_path.trim_start_matches('/'));
        if !dir_path.ends_with('/') {
            dir_path.push('/');
        }

        Self {
            root_path,
            url_path,
            dir_path,
        }
    }

    pub fn url_path(&self) -> &str {
        &self.url_path
    }

    pub fn dir_path(&self) -> &str {
        &self.dir_path
    }

    pub async fn is_dir(&self) -> bool {
        match fs::metadata(&self.dir_path).await {
            Ok(md) if md.is_dir() => true,
            _ => false,
        }
    }
}
