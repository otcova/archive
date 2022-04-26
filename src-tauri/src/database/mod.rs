#![allow(dead_code)]

#[cfg(test)]
mod test_utils;

mod error;
mod lock;
mod serializer;

use self::{error::Result, lock::Lock};
use std::path::PathBuf;

pub struct Database<T> {
    lock: Lock,
    data: T,
}

impl Database {
}