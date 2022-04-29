#![allow(dead_code)]

#[cfg(test)]
mod test_utils;

mod error;
mod lock;
mod serializer;
mod history;

use self::{error::Result, lock::Lock};
use std::path::PathBuf;

pub struct Database<T: Default> {
    lock: Lock,
    data: T,
}

impl<T: Default> Database<T> {
    pub fn open(path: PathBuf) -> Result<Self> {
        let lock = Lock::directory(&path)?;
        let data: T = history::open(&path.join("data-history"))?;
        Ok(Self { lock, data })
    }
    pub fn create(path: PathBuf) -> Result<Self> {
        let lock = Lock::directory(&path)?;
        let data: T = history::create(&path.join("data-history"), Default::default())?;
        Ok(Self { lock, data })
    }
}