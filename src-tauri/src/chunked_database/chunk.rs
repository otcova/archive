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
    pub fn open(path: &PathBuf) -> Result<Self>  {
        match Self::create_chunk(path) {
            Err(err) => match *err {
                ErrorKind::AlreadyExist => Self::open_chunk(path),
                err => err.into(),
            },
            database_chunk => database_chunk,
        }
    }
    fn open_chunk(path: &PathBuf) -> Result<Self> {
        Ok(Self {
            database: Database::open(path)?,
        })
    }
    fn create_chunk(path: &PathBuf) -> Result<Self> {
        Ok(Self {
            database: Database::create(path)?,
        })
    }

    pub fn min_sort(&self) -> BinaryHeap<Id> {
        todo!();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_utils::*;

    #[derive(Debug, Serialize, Deserialize, Default, Clone, Copy)]
    struct Data(i32);
    impl Item for Data {
        fn date(&self) -> i32 {
            self.0
        }
    }

    #[test]
    fn create_database_chunk_on_empty_dir() {
        let tempdir = TempDir::new();
        let chunk = Chunk::<Data>::open(&tempdir.path).unwrap();
        assert_eq!(chunk.database.data.items.len(), 0);
    }
    
    #[test]
    fn create_and_open_database_chunk() {
        let tempdir = TempDir::new();
        {
            let chunk = Chunk::<Data>::open(&tempdir.path).unwrap();
            assert_eq!(chunk.database.data.items.len(), 0);
        }
        {
            let chunk = Chunk::<Data>::open(&tempdir.path).unwrap();
            assert_eq!(chunk.database.data.items.len(), 0);
        }
    }
}
