#![allow(dead_code)]

#[cfg(test)]
mod test_utils;

mod lock;
mod serializer;

use self::lock::Lock;
use std::{path::PathBuf, io};

pub struct Database {
    lock: Lock,
    path: PathBuf,
}

impl Database {
    pub fn init(path: PathBuf) -> io::Result<Self> {
        let lock = Lock::new(&path)?;
        Ok(Self { lock, path })
    }
}
