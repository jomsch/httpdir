// MetaFile - Meta information about a file
// File can be a directory or standard file
#[derive(Clone, Debug)]
pub struct MetaFile {
    file_name: String,
    is_dir: bool,
}

impl MetaFile {
    pub async fn try_from(entry: async_std::fs::DirEntry) -> async_std::io::Result<Self> {
        let file_name = entry.file_name().to_string_lossy().into();
        let metadata = entry.metadata().await?;
        let is_dir = metadata.is_dir();

        Ok(MetaFile { file_name, is_dir })
    }

    pub fn name(&self) -> &str {
        &self.file_name
    }

    pub fn is_dir(&self) -> bool {
        self.is_dir
    }
}

#[derive(Default, Debug, Clone)]
pub struct Directory {
    dirs: Vec<MetaFile>,
    files: Vec<MetaFile>,
}

impl Directory {
    pub fn push(&mut self, file: MetaFile) {
        match file.is_dir {
            true => self.dirs.push(file),
            false => self.files.push(file),
        }
    }
}

impl IntoIterator for Directory {
    type Item = MetaFile;
    type IntoIter =
        std::iter::Chain<std::vec::IntoIter<Self::Item>, std::vec::IntoIter<Self::Item>>;

    fn into_iter(self) -> Self::IntoIter {
        let dirs = self.dirs.into_iter();
        let files = self.files.into_iter();
        dirs.chain(files)
    }
}
