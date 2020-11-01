use std::cmp::Ordering;

use structopt::clap;

// MetaFile - Meta information about a file
// File can be a directory or standard file
#[derive(Clone, Debug, PartialEq)]
pub struct MetaFile {
    file_name: String,
    is_dir: bool,
}

impl MetaFile {

    // Helper function for testing purpose.
    fn new<S: ToString>(name: S, is_dir: bool) -> Self {
        Self {
            file_name: name.to_string(),
            is_dir,
        }
    }

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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FileGrouping {
    DirectoriesFirst,
    FilesFirst,
    Mixed,
}

impl std::str::FromStr for FileGrouping {
    type Err = clap::Error;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let order = match s {
            "directories" => FileGrouping::DirectoriesFirst, 
            "files" => FileGrouping::FilesFirst,
            "none" => FileGrouping::Mixed,
            _ => return Err(clap::Error::with_description("Could not parse value", clap::ErrorKind::InvalidValue))
        };

        Ok(order)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FileSort {
    Alphabetical,
    RevAlphabetical
}

impl std::str::FromStr for FileSort {
    type Err = clap::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sort = match s {
            "atoz" => FileSort::Alphabetical,
            "ztoa" => FileSort::RevAlphabetical,
            _ => return Err(clap::Error::with_description("Could not parse value", clap::ErrorKind::InvalidValue))
        };

        Ok(sort)
    }
}

#[derive(Debug, Clone)]
pub struct Directory {
    files: Vec<MetaFile>,
    order: FileGrouping,
    sort: FileSort,
    include_dotfiles: bool,
}

impl Default for Directory {
    fn default() -> Self {
        Self {
            files: Default::default(),
            order: FileGrouping::DirectoriesFirst,
            sort: FileSort::Alphabetical,
            include_dotfiles: false,
        }
    }
}

struct DirectoryVector  {
    dirs: Vec<MetaFile>,
    files: Vec<MetaFile>,
    seperator_idx: usize,
}


// Function for weighted compare of MetaFiles
fn compare(a: &MetaFile, b: &MetaFile, order: &FileGrouping, sort: &FileSort) -> Ordering {
    let sort_cmp = || {
        match sort {
            FileSort::Alphabetical => a.name() < b.name(),
            FileSort::RevAlphabetical => a.name() > b.name()
        }
    };
    
    match order {
        FileGrouping::DirectoriesFirst if (b.is_dir() && !a.is_dir()) => Ordering::Greater,
        FileGrouping::DirectoriesFirst if (a.is_dir() && !b.is_dir()) =>  Ordering::Less, 
        FileGrouping::FilesFirst if (!b.is_dir() && a.is_dir()) => Ordering::Greater,
        FileGrouping::FilesFirst if (!a.is_dir() && b.is_dir()) => Ordering::Less,
        _ if sort_cmp() => Ordering::Less, 
        _ => Ordering::Greater
    }
}

impl Directory {

    pub fn new(order: FileGrouping, sort: FileSort, include_dotfiles: bool) -> Self {
        Self {
            files: Vec::new(),
            order,
            sort,
            include_dotfiles
        }
    }


    // insert the MetaFile to the position depending on DirOrder and FileSort
    pub fn push(&mut self, file: MetaFile) {
        if !self.include_dotfiles && file.name().starts_with('.'){
            return
        }

        self.files.push(file);
        let order = self.order.clone();
        let sort = self.sort.clone();
        self.files.sort_by(move |a, b| compare(a, b, &order, &sort));

    }

    pub fn get(&self, index: usize) -> Option<&MetaFile> {
        self.files.get(index) 
    }

    pub fn len(&self) -> usize {
            self.files.len()
    }


    pub fn include_dotfiles(&mut self, b: bool) {
        self.include_dotfiles = b;
    }
}

impl IntoIterator for Directory {
    type Item = MetaFile;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.files.into_iter()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_compare_alpha() {
        let a = MetaFile::new("AAA", false);
        let b = MetaFile::new("AAB", false);
        let order = FileGrouping::FilesFirst;
        let sort = FileSort::Alphabetical;
        assert_eq!(compare(&a, &b, &order, &sort), Ordering::Less);
        assert_eq!(compare(&b, &a, &order, &sort), Ordering::Greater);
    }


    #[test]
    fn test_compare_rev_alpha() {
        let a = MetaFile::new("AAA", false);
        let b = MetaFile::new("AAB", false);
        let order = FileGrouping::FilesFirst;
        let sort = FileSort::RevAlphabetical;
        assert_eq!(compare(&a, &b, &order, &sort), Ordering::Greater);
        assert_eq!(compare(&b, &a, &order, &sort), Ordering::Less);
    }

    #[test]
    fn test_compare_mixed() {
        let a = MetaFile::new("AAA", false);
        let b = MetaFile::new("AAB", true);
        let order = FileGrouping::Mixed;
        let sort = FileSort::Alphabetical;
        assert_eq!(compare(&a, &b, &order, &sort), Ordering::Less);
        assert_eq!(compare(&b, &a, &order, &sort), Ordering::Greater);
    }
    
    #[test]
    fn test_compare_directory_first() {
        let a = MetaFile::new("AAA", false);
        let b = MetaFile::new("BBB", true);
        let order = FileGrouping::DirectoriesFirst;
        let sort = FileSort::Alphabetical;
        assert_eq!(compare(&a, &b, &order, &sort), Ordering::Greater);
        assert_eq!(compare(&b, &a, &order, &sort), Ordering::Less);
    }

    #[test]
    fn test_compare_file_first() {
        let a = MetaFile::new("AAA", false);
        let b = MetaFile::new("BBB", true);
        let order = FileGrouping::FilesFirst;
        let sort = FileSort::Alphabetical;
        assert_eq!(compare(&a, &b, &order, &sort), Ordering::Less);
        assert_eq!(compare(&b, &a, &order, &sort), Ordering::Greater);
    }
}
