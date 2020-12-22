use crate::{
    error::*,
    file_type::FileType,
    iter::{FilesIter, PathsIter},
};

use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct File<T> {
    pub path: PathBuf,
    pub file_type: FileType<T>,
    pub extra: Option<T>,
}

impl<'a, T> File<T> {
    /// Create `File` from arguments
    ///
    /// This function will panic if you pass a path with multiple components to
    /// it, because it breaks iterators functionality.
    pub fn new(path: impl AsRef<Path>, file_type: FileType<T>) -> Self {
        let path = path.as_ref().to_path_buf();
        File {
            path,
            file_type,
            extra: None,
        }
    }

    /// Create `File` reading from the `path`
    pub fn new_from_path(path: impl AsRef<Path>, follow_symlinks: bool) -> FSResult<Self> {
        let file_type = FileType::from_path(&path, follow_symlinks)?;
        let result = File::new(path, file_type);

        Ok(result)
    }

    /// Iterator of all `File`s in the structure
    pub fn files(&'a self) -> FilesIter<'a, T> {
        FilesIter::new(self)
    }

    /// Shorthand for `self.files().paths()`, see link to `.paths()` method
    pub fn paths(&'a self) -> PathsIter<'a, T> {
        self.files().paths()
    }

    pub fn children(&self) -> Option<&Vec<File<T>>> {
        self.file_type.children()
    }
}

impl<T: Default> Default for File<T> {
    fn default() -> Self {
        File {
            path: Default::default(),
            file_type: Default::default(),
            extra: Default::default(),
        }
    }
}
