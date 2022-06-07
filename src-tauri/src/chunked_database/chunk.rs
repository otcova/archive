pub use super::Id;
use super::*;
use crate::database::Database;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{collections::BinaryHeap, path::PathBuf};

pub trait Item: Serialize + DeserializeOwned + Default + Clone {
    fn date(&self) -> i32;
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DataType<T: Serialize> {
    pub items: IdMap<T>,
}

#[derive(Debug)]
pub struct Chunk<T: Item> {
    pub database: Database<DataType<T>>,
}

impl<T: Item> Chunk<T> {
    /// Open on a empty direcory will create a blanck database
    pub fn open(path: &PathBuf) -> Result<Self> {
        match Database::open(path) {
            Ok(database) => Ok(Self { database }),
            Err(err) => match *err {
                ErrorKind::NotFound => Ok(Self {
                    database: Database::create(path)?,
                }),
                err => err.into(),
            },
        }
    }

    pub fn min_sort(&self) -> BinaryHeap<Id> {
        todo!();
    }
}

