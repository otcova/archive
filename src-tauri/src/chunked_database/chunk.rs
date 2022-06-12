pub use super::Id;
use super::*;
use crate::database::Database;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::path::PathBuf;

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
        match Self::create_database(path) {
            Err(err) => match *err {
                ErrorKind::AlreadyExist => Self::open_database(path),
                err => err.into(),
            },
            database_chunk => database_chunk,
        }
    }
    fn open_database(path: &PathBuf) -> Result<Self> {
        Ok(Self {
            database: Database::open(path)?,
        })
    }
    fn create_database(path: &PathBuf) -> Result<Self> {
        Ok(Self {
            database: Database::create(path)?,
        })
    }

    pub fn pop_oldest(&mut self) -> Option<T> {
        let mut oldest_id = 0;
        let mut oldest_date = i32::MAX;

        for (id, item) in self.database.data.items.iter() {
            let item_date = item.date();
            if item_date < oldest_date {
                oldest_date = item_date;
                oldest_id = id;
            }
        }
        if oldest_date == i32::MAX {
            return None;
        }
        self.database.data.items.pop(oldest_id)
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

    #[test]
    fn create_store_and_open_database_chunk() {
        let tempdir = TempDir::new();
        {
            let mut chunk = Chunk::<Data>::open(&tempdir.path).unwrap();
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
        let mut chunk = Chunk::<Data>::open(&tempdir.path).unwrap();
        assert_eq!(chunk.pop_oldest(), None);
    }

    #[test]
    fn pop_oldest() {
        let tempdir = TempDir::new();
        let mut chunk = Chunk::<Data>::open(&tempdir.path).unwrap();
        chunk.database.data.items.push(Data(123));
        assert_eq!(chunk.pop_oldest(), Some(Data(123)));
        assert_eq!(chunk.pop_oldest(), None);
        assert_eq!(chunk.pop_oldest(), None);
    }
}
