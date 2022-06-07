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
}

impl<T: Item> ChunkedDatabase<T> {
    pub fn open(path: &PathBuf) -> Result<Self> {
        Ok(Self {
            dynamic: Chunk::open(&path.join("dynamic"))?,
            ancient: Chunk::open(&path.join("ancient"))?,
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

    // if dynamic database has more than max items,
    // then the oldest items are moved to the ancient database
    // pub fn transfer_old_items(&mut self, min: usize, max: usize) {
    //     let dyn_len = self.dynamic.database.data.items.len();
    //     if dyn_len > max {
    //         let mut heap = self.dynamic.min_sort();
    //         for _ in 0..dyn_len - min {
    //             let id = heap.pop().unwrap();
    //             let item = self.dynamic.database.data.items.read(id).clone().unwrap();
    //             self.ancient.database.data.items.push(item);
    //             self.dynamic.database.data.items.delete(id);
    //         }
    //     }
    // }
}
