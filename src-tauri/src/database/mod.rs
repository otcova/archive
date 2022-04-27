#![allow(dead_code)]

#[cfg(test)]
mod test_utils;

mod error;
mod lock;
mod serializer;
mod history;

use self::{error::Result, lock::Lock};
use std::path::PathBuf;

pub struct Database<T> {
    lock: Lock,
    data: T,
}

impl<T> Database<T> {
    pub fn open(path: PathBuf) -> Result<Self> {
        let lock = Lock::directory(&path)?;
        let data: T = history::load_data(&path.join("data-history"))?;
        Ok(Self { lock, data })
    }
}