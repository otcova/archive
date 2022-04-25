use fs2::FileExt;
use std::{fs::File, io, path::PathBuf};

pub struct Lock {
    lock_file: File,
}

impl Lock {
    pub fn new(path: &PathBuf) -> io::Result<Self> {
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
    fn lock_empty_directory() {
        let tempdir = TempDir::new();
        let lock = Lock::new(&tempdir.path);
        assert_eq!(lock.is_ok(), true);
    }

    #[test]
    fn two_locks_on_same_directory() {
        let tempdir = TempDir::new();
        let lock1 = Lock::new(&tempdir.path);
        let lock2 = Lock::new(&tempdir.path);
        assert_eq!(lock1.is_ok(), true);
        assert_eq!(lock2.is_ok(), false);
    }

    #[test]
    fn release_lock_on_drop() {
        let tempdir = TempDir::new();
        {
            let lock = Lock::new(&tempdir.path);
            assert_eq!(lock.is_ok(), true);
        }
        {
            let lock = Lock::new(&tempdir.path);
            assert_eq!(lock.is_ok(), true);
        }
    }
}
