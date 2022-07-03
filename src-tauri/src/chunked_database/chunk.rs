use super::*;
pub use crate::collections::Id;
use crate::collections::*;
use crate::database::{Database, RollbackDateInfo};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::path::PathBuf;

pub trait Item: Serialize + DeserializeOwned + Clone + Sync + Send {
    fn date(&self) -> i32;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataType<T: Serialize + Clone + Send + Sync> {
    pub items: IdMap<T>,
}

impl<T: Serialize + Clone + Send + Sync> Default for DataType<T> {
    fn default() -> DataType<T> {
        DataType {
            items: IdMap::default(),
        }
    }
}

#[derive(Debug)]
pub struct Chunk<T: Item> {
    pub database: Database<DataType<T>>,
}

impl<T: Item> Chunk<T> {
    pub fn open(path: &PathBuf) -> Result<Self> {
        Ok(Self {
            database: Database::open(path)?,
        })
    }
    pub fn create(path: &PathBuf) -> Result<Self> {
        Ok(Self {
            database: Database::create(path)?,
        })
    }
    pub fn rollback(path: &PathBuf) -> Result<Self> {
        Ok(Self {
            database: Database::rollback(path)?,
        })
    }
    pub fn rollback_info(path: &PathBuf) -> Result<RollbackDateInfo> {
        Database::<DataType<T>>::rollback_info(path)
    }

    pub fn pop_oldest(&mut self) -> Option<T> {
        let mut oldest_id = None;
        let mut oldest_date = i32::MAX;

        for (id, item) in self.database.data.items.iter() {
            let item_date = item.date();
            if item_date < oldest_date {
                oldest_date = item_date;
                oldest_id = Some(id);
            }
        }

        self.database.data.items.pop(oldest_id?)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_utils::*;

    #[derive(Debug, Serialize, Deserialize, Default, Clone, Copy, PartialEq, Eq)]
    struct Data(i32);
    impl Item for Data {
        fn date(&self) -> i32 {
            self.0
        }
    }

    #[test]
    fn create_database_chunk_on_empty_dir() {
        let tempdir = TempDir::new();
        let chunk = Chunk::<Data>::create(&tempdir.path).unwrap();
        assert_eq!(chunk.database.data.items.len(), 0);
    }

    #[test]
    fn create_and_open_database_chunk() {
        let tempdir = TempDir::new();
        {
            let chunk = Chunk::<Data>::create(&tempdir.path).unwrap();
            assert_eq!(chunk.database.data.items.len(), 0);
        }
        {
            let chunk = Chunk::<Data>::open(&tempdir.path).unwrap();
            assert_eq!(chunk.database.data.items.len(), 0);
        }
    }

    #[test]
    fn create_store_and_open_database_chunk() {
        let tempdir = TempDir::new();
        {
            let mut chunk = Chunk::<Data>::create(&tempdir.path).unwrap();
            chunk.database.data.items.push(Data(123));
            assert_eq!(chunk.database.data.items.len(), 1);
        }
        {
            let chunk = Chunk::<Data>::open(&tempdir.path).unwrap();
            assert_eq!(chunk.database.data.items.len(), 1);
        }
    }

    #[test]
    fn pop_oldest_from_empty() {
        let tempdir = TempDir::new();
        let mut chunk = Chunk::<Data>::create(&tempdir.path).unwrap();
        assert_eq!(chunk.pop_oldest(), None);
    }

    #[test]
    fn pop_oldest() {
        let tempdir = TempDir::new();
        let mut chunk = Chunk::<Data>::create(&tempdir.path).unwrap();
        chunk.database.data.items.push(Data(123));
        assert_eq!(chunk.pop_oldest(), Some(Data(123)));
        assert_eq!(chunk.pop_oldest(), None);
        assert_eq!(chunk.pop_oldest(), None);
    }
}
