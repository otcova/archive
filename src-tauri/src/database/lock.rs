use fs2::FileExt;
use std::{fs::File, path::PathBuf};
use super::error::Result;

#[derive(Debug)]
pub struct Lock {
    lock_file: File,
}

impl Lock {
    pub fn directory(path: &PathBuf) -> Result<Self> {
        let lock_file = File::create(path.join(".lock"))?;
        lock_file.try_lock_exclusive()?;
        Ok(Self { lock_file })
    }
}

#[cfg(test)]
mod tests {
    use super::Lock;
    use crate::database::test_utils::TempDir;
    
    #[test]
    fn lock_non_existing_directory() {
        let tempdir = TempDir::new();
        let error = Lock::directory(&tempdir.path.join("not-a-real-path"));
        assert_eq!(format!("{:?}", error), "Err(NotFound)");
    }
    
    #[test]
    fn lock_empty_directory() {
        let tempdir = TempDir::new();
        let lock = Lock::directory(&tempdir.path);
        assert!(lock.is_ok());
    }

    #[test]
    fn two_locks_on_same_directory() {
        let tempdir = TempDir::new();
        let lock1 = Lock::directory(&tempdir.path);
        let lock2 = Lock::directory(&tempdir.path);
        assert!(lock1.is_ok());
        assert!(lock2.is_err());
    }

    #[test]
    fn release_lock_on_drop() {
        let tempdir = TempDir::new();
        {
            let lock = Lock::directory(&tempdir.path);
            assert!(lock.is_ok());
        }
        {
            let lock = Lock::directory(&tempdir.path);
            assert!(lock.is_ok());
        }
    }
}
