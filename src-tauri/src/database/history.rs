use super::error::{ErrorKind, Result};
use std::path::PathBuf;

pub fn load_data<T>(_dir: &PathBuf) -> Result<T> {
    Err(ErrorKind::NotFound.into())
}

#[cfg(test)]
mod tests {
    use super::load_data;
    use crate::database::test_utils::TempDir;

    #[test]
    fn load_data_from_empty_dir() {
        let tempdir = TempDir::new();
        let result = load_data::<usize>(&tempdir.path);
        assert_eq!(format!("{:?}", result), "Err(NotFound)");
    }
}
