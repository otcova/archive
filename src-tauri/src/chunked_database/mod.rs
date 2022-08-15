mod chunk;

pub use crate::collections::*;
pub use crate::database::RollbackDateInfo;
use crate::error::*;
pub use chunk::Item;
use chunk::*;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Data is composed of items, each item have a 'date' associated
/// and is stored in on of the two interal databases in relation of that date
///
/// In the dynamic database is stored the newest data.
/// This way, you can skip scaning very old data.
///
/// In the ancient database are stored all the data considered old.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Uid {
    DYNAMIC(chunk::Id),
    ANCIENT(chunk::Id),
}

#[derive(Debug)]
pub struct ChunkedDatabase<T: Item + Send + Sync> {
    dynamic: Chunk<T>,
    ancient: Chunk<T>,
    max_dynamic_len: usize,
}

impl<T: Item + Send + Sync> ChunkedDatabase<T> {
    pub fn open(path: &PathBuf, max_dynamic_len: usize) -> Result<Self> {
        Ok(Self {
            dynamic: Chunk::open(&path.join("dynamic"))?,
            ancient: Chunk::open(&path.join("ancient"))?,
            max_dynamic_len,
        })
    }

    pub fn create(path: &PathBuf, max_dynamic_len: usize) -> Result<Self> {
        Ok(Self {
            dynamic: Chunk::create(&path.join("dynamic"))?,
            ancient: Chunk::create(&path.join("ancient"))?,
            max_dynamic_len,
        })
    }

    pub fn rollback(path: &PathBuf, max_dynamic_len: usize) -> Result<Self> {
        Ok(Self {
            dynamic: Chunk::rollback(&path.join("dynamic"))?,
            ancient: Chunk::rollback(&path.join("ancient"))?,
            max_dynamic_len,
        })
    }

    pub fn rollback_info(path: &PathBuf) -> Result<RollbackDateInfo> {
        let dynamic_info = Chunk::<T>::rollback_info(&path.join("dynamic"))?;
        let ancient_info = Chunk::<T>::rollback_info(&path.join("ancient"))?;
        Ok(RollbackDateInfo {
            newest_instant: dynamic_info
                .newest_instant
                .max(&ancient_info.newest_instant),
            rollback_instant: dynamic_info
                .rollback_instant
                .min(&ancient_info.rollback_instant),
        })
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = (Uid, &'a T)> + 'a {
        let iter = self.dynamic.iter();
        iter.map(|item| (Uid::DYNAMIC(item.id), item.data))
    }

    pub fn iter_ancient(&self) -> impl Iterator<Item = (Uid, &T)> {
        let iter = self.ancient.iter();
        iter.map(|item| (Uid::ANCIENT(item.id), item.data))
    }

    pub fn iter_all<'a>(&'a self) -> impl Iterator<Item = (Uid, &'a T)> + 'a {
        self.iter().chain(self.iter_ancient())
    }

    pub fn delete(&mut self, id: Uid) {
        match id {
            Uid::DYNAMIC(id) => self.dynamic.delete(id),
            Uid::ANCIENT(id) => self.ancient.delete(id),
        }
    }
    pub fn push(&mut self, item: T) -> Uid {
        Uid::DYNAMIC(self.dynamic.push(item))
    }
    pub fn update(&mut self, id: Uid, item: T) {
        match id {
            Uid::DYNAMIC(id) => self.dynamic.update(id, item),
            Uid::ANCIENT(id) => self.ancient.update(id, item),
        }
    }
    pub fn read(&self, id: Uid) -> Option<&T> {
        match id {
            Uid::DYNAMIC(id) => self.dynamic.ref_data(id),
            Uid::ANCIENT(id) => self.ancient.ref_data(id),
        }
    }
    pub fn len(&self) -> usize {
        self.dynamic.len() + self.ancient.len()
    }

    /// Moves items from the dynamic chunk to the ancient chunk to satisfy 'max_dynamic_len'
    fn move_old_items(&mut self) {
        if self.dynamic.len() > self.max_dynamic_len {
            println!(
                "Moving {} expedient to ancient database",
                self.dynamic.len() - self.max_dynamic_len
            );
            while self.dynamic.len() > self.max_dynamic_len {
                self.ancient.push(
                    self.dynamic
                        .pop_oldest()
                        .expect("Dynamic len is > 0 but pop_oldest didn't find any"),
                );
            }
            println!(
                "Dinamic database: {}  Ancient database: {}",
                self.dynamic.len(),
                self.ancient.len()
            );
        }
    }

    pub fn save(&mut self) -> Result<()> {
        self.move_old_items();
        self.dynamic.save()?;
        self.ancient.save()?;
        Ok(())
    }
}

impl<T: Item + Send + Sync> Drop for ChunkedDatabase<T> {
    fn drop(&mut self) {
        self.move_old_items();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_utils::*;

    #[derive(Debug, Serialize, Deserialize, Default, Clone, Copy, PartialEq, Eq)]
    struct Data(i64);
    impl Item for Data {
        fn date(&self) -> i64 {
            self.0
        }
    }

    #[test]
    fn push_len_and_read() {
        let tempdir = TempDir::new();

        let mut db = ChunkedDatabase::<Data>::create(&tempdir.path, 100).unwrap();
        let id_54 = db.push(Data(54));
        let id_13 = db.push(Data(13));
        let id_223 = db.push(Data(223));

        assert_eq!(3, db.len());

        assert_eq!(Some(&Data(13)), db.read(id_13));
        assert_eq!(Some(&Data(223)), db.read(id_223));
        assert_eq!(Some(&Data(54)), db.read(id_54));
    }

    #[test]
    fn move_old_items() {
        let tempdir = TempDir::new();

        let mut db = ChunkedDatabase::<Data>::create(&tempdir.path, 2).unwrap();
        db.push(Data(54));
        db.push(Data(74));
        db.push(Data(13));
        db.push(Data(223));
        db.move_old_items();

        assert_eq!(4, db.len());
        assert_eq!(2, db.iter().count());
        assert_eq!(2, db.iter_ancient().count());
    }

    #[test]
    fn move_old_items_on_drop() {
        let tempdir = TempDir::new();
        {
            let mut db = ChunkedDatabase::<Data>::create(&tempdir.path, 2).unwrap();
            db.push(Data(54));
            db.push(Data(74));
            db.push(Data(13));
            db.push(Data(223));
        }

        let db = ChunkedDatabase::<Data>::open(&tempdir.path, 2).unwrap();

        assert_eq!(4, db.len());
        assert_eq!(2, db.iter().count());
        assert_eq!(2, db.iter_ancient().count());
    }
}
