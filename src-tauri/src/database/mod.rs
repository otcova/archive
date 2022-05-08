#![allow(dead_code)]

#[cfg(test)]
mod test_utils;

mod error;
mod lock;
mod serializer;
mod backup;

use self::{error::Result, lock::Lock};
use std::path::PathBuf;
use serde::{Serialize, de::DeserializeOwned};

pub struct Database<T: Default> {
    lock: Lock,
    data: T,
}

impl<T: Default + DeserializeOwned + Serialize> Database<T> {
    pub fn open(path: PathBuf) -> Result<Self> {
        let lock = Lock::directory(&path)?;
        let data: T = backup::load_newest(&path.join("data-backups"))?;
        Ok(Self { lock, data })
    }
    pub fn create(path: PathBuf) -> Result<Self> {
        let lock = Lock::directory(&path)?;
        let data: T = Default::default();
        Ok(Self { lock, data })
    }
}

#[cfg(test)]
mod tests {
    
}