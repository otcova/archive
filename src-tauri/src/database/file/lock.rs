use crate::error::*;
use fs2::FileExt;
use std::{fs::File, path::PathBuf};

#[derive(Debug)]
pub struct Lock {
    lock_file: File,
}

impl Lock {
    pub fn directory(path: &PathBuf) -> Result<Self> {
        let lock_file =
            File::create(path.join(".lock")).map_err::<Error, _>(|error| match error.kind() {
                std::io::ErrorKind::NotFound => ErrorKind::NotFound.into(),
                _ => ErrorKind::Collision.into(),
            })?;
        lock_file
            .try_lock_exclusive()
            .map_err::<Error, _>(|_| ErrorKind::Collision.into())?;
        Ok(Self { lock_file })
    }
}

#[cfg(test)]
mod tests {
    use super::Lock;
    use crate::test_utils::TempDir;

    #[test]
    fn lock_non_existing_directory() {
        let tempdir = TempDir::new();
        let error = Lock::directory(&tempdir.path.join("not-a-real-path"));
        assert_eq!("Err(NotFound)", format!("{:?}", error));
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
        assert_eq!("Err(Collision)", format!("{:?}", lock2));
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
