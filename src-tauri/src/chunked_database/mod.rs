mod chunk;
mod collections;

use crate::error::*;
use chunk::*;
pub use collections::*;
use std::path::PathBuf;

/// Data is composed of items, each item have a 'date' associated
/// and is stored in on of the two interal databases in relation of that date
///
/// In the dynamic database is stored the newest data.
/// This way, you can skip scaning very old data.
///
/// In the ancient database are stored all the data considered old.

pub enum Uid {
    DYNAMIC(chunk::Id),
    ANCIENT(chunk::Id),
}

pub struct ChunkedDatabase<T: Item> {
    dynamic: Chunk<T>,
    ancient: Chunk<T>,
    max_dynamic_len: usize,
}

impl<T: Item> ChunkedDatabase<T> {
    pub fn open(path: &PathBuf, max_dynamic_len: usize) -> Result<Self> {
        Ok(Self {
            dynamic: Chunk::open(&path.join("dynamic"))?,
            ancient: Chunk::open(&path.join("ancient"))?,
            max_dynamic_len,
        })
    }

    pub fn iter(&self) -> IdMapIter<T> {
        self.dynamic.database.data.items.iter()
    }

    pub fn delete(&mut self, id: Uid) {
        match id {
            Uid::DYNAMIC(id) => self.dynamic.database.data.items.delete(id),
            Uid::ANCIENT(id) => self.ancient.database.data.items.delete(id),
        }
    }
    pub fn push(&mut self, item: T) -> Uid {
        Uid::DYNAMIC(self.dynamic.database.data.items.push(item))
    }
    pub fn update(&mut self, id: Uid, item: T) {
        match id {
            Uid::DYNAMIC(id) => self.dynamic.database.data.items.update(id, item),
            Uid::ANCIENT(id) => self.ancient.database.data.items.update(id, item),
        }
    }
    pub fn read(&self, id: Uid) -> &Option<T> {
        match id {
            Uid::DYNAMIC(id) => self.dynamic.database.data.items.read(id),
            Uid::ANCIENT(id) => self.ancient.database.data.items.read(id),
        }
    }
    pub fn len(&self) -> usize {
        self.dynamic.database.data.items.len() + self.ancient.database.data.items.len()
    }

    /// Moves items from the dynamic chunk to the ancient chunk to satisfy 'max_dynamic_len'
    fn move_old_items(&mut self) {}
}

impl<T: Item> Drop for ChunkedDatabase<T> {
    fn drop(&mut self) {
        self.move_old_items();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_utils::*;
    use serde::*;

    #[derive(Debug, Serialize, Deserialize, Default, Clone, Copy, PartialEq, Eq)]
    struct Data(i32);
    impl Item for Data {
        fn date(&self) -> i32 {
            self.0
        }
    }

    #[test]
    fn push_len_and_read() {
        let tempdir = TempDir::new();
        
        let mut db = ChunkedDatabase::<Data>::open(&tempdir.path, 100).unwrap();
        let id_54 = db.push(Data(54));
        let id_13 = db.push(Data(13));
        let id_223 = db.push(Data(223));
        
        assert_eq!(3, db.len());
        
        assert_eq!(Some(Data(13)), *db.read(id_13));
        assert_eq!(Some(Data(223)), *db.read(id_223));
        assert_eq!(Some(Data(54)), *db.read(id_54));
    }
}
