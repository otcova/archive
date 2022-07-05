use serde::{Deserialize, Serialize};
use std::{
    mem::size_of,
    slice::{Iter, IterMut},
};

/// Id.0 is the index to the data and Id.1 is an identifier
/// to check if the data has been replaced
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Id {
    pub index: usize,
    pub identifier: usize,
}

impl Id {
    fn new(index: usize, identifier: usize) -> Self {
        Self { index, identifier }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Item<T: Send + Sync> {
    identifier: usize,
    data: Option<T>,
}

impl<T: Send + Sync> Item<T> {
    fn new(identifier: usize, data: T) -> Self {
        Self {
            identifier,
            data: Some(data),
        }
    }
    fn is_some(&self) -> bool {
        self.data.is_some()
    }
    fn clean(&mut self) {
        self.identifier = 0;
        self.data = None;
    }
    fn take(&mut self) -> Option<T> {
        if self.is_some() {
            self.data.take()
        } else {
            None
        }
    }
    fn ref_data(&self) -> Option<&T> {
        if self.is_some() {
            self.data.as_ref()
        } else {
            None
        }
    }
    fn mut_data(&mut self) -> Option<&mut T> {
        if self.is_some() {
            self.data.as_mut()
        } else {
            None
        }
    }
    fn update(&mut self, data: T) {
        self.data = Some(data)
    }
}

impl<T: Clone + Send + Sync> Item<T> {
    fn clone_data(&self) -> Option<T> {
        if self.is_some() {
            self.data.clone()
        } else {
            None
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct IdMap<T: Send + Sync> {
    data: Vec<Item<T>>,
    empty_indexes: Vec<usize>,
    last_identifier: usize,
}

#[derive(Debug, Serialize)]
pub struct IdMapSerialize<T: Serialize + Send + Sync> {
    data: Vec<Item<T>>,
    empty_indexes: Vec<usize>,
    last_identifier: usize,
}

impl<T: Serialize + Send + Sync> Serialize for IdMap<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let data: &IdMapSerialize<T> = unsafe { std::mem::transmute(self) };
        data.serialize(serializer)
    }
}

impl<T: Send + Sync> Default for IdMap<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Send + Sync> IdMap<T> {
    pub fn new() -> Self {
        Self {
            data: vec![],
            empty_indexes: vec![],
            last_identifier: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.data.len() - self.empty_indexes.len()
    }

    pub fn push(&mut self, item: T) -> Id {
        self.last_identifier += 1;
        if let Some(reused_id) = self.empty_indexes.pop() {
            self.data[reused_id] = Item::new(self.last_identifier, item);
            return Id::new(reused_id, self.last_identifier);
        }
        self.data.push(Item::new(self.last_identifier, item));
        Id::new(self.data.len() - 1, self.last_identifier)
    }

    pub fn pop(&mut self, id: Id) -> Option<T> {
        if self.exists(id) {
            self.empty_indexes.push(id.index);
            self.data[id.index].take()
        } else {
            None
        }
    }

    pub fn delete(&mut self, id: Id) {
        if self.exists(id) {
            self.data[id.index].clean();
            self.empty_indexes.push(id.index);
        }
    }

    pub fn exists(&self, id: Id) -> bool {
        if id.index < self.data.len() {
            self.data[id.index].identifier == id.identifier
        } else {
            false
        }
    }

    pub fn iter(&self) -> IdMapIter<T> {
        IdMapIter {
            data_iter: self.data.iter(),
            index: 0,
        }
    }

    pub fn iter_mut(&mut self) -> IdMapIterMut<T> {
        IdMapIterMut {
            data_iter: self.data.iter_mut(),
            index: 0,
        }
    }

    pub fn take_iter<'a>(&'a mut self) -> impl Iterator<Item = T> + 'a {
        self.data.iter_mut().map(|item| item.take()).flatten()
    }

    pub fn update(&mut self, id: Id, item: T) {
        if id.index < self.data.len() {
            self.data[id.index].update(item)
        }
    }

    pub fn ref_data(&self, id: Id) -> Option<&T> {
        if self.exists(id) {
            self.data[id.index].ref_data()
        } else {
            None
        }
    }
}

impl<T: Clone + Send + Sync> IdMap<T> {
    pub fn clone_data(&self, id: Id) -> Option<T> {
        if self.exists(id) {
            self.data[id.index].clone_data()
        } else {
            None
        }
    }
}

pub struct IdMapIter<'a, T: Send + Sync> {
    data_iter: Iter<'a, Item<T>>,
    index: usize,
}

impl<'a, T: Send + Sync> Iterator for IdMapIter<'a, T> {
    type Item = (Id, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(item) = self.data_iter.next() {
            self.index += 1;
            if item.is_some() {
                return Some((
                    Id {
                        index: self.index - 1,
                        identifier: item.identifier,
                    },
                    item.ref_data().unwrap(),
                ));
            }
        }
        None
    }
}

pub struct IdMapIterMut<'a, T: Send + Sync> {
    data_iter: IterMut<'a, Item<T>>,
    index: usize,
}

impl<'a, T: Send + Sync> Iterator for IdMapIterMut<'a, T> {
    type Item = (Id, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(item) = self.data_iter.next() {
            self.index += 1;
            if item.is_some() {
                return Some((
                    Id {
                        index: self.index - 1,
                        identifier: item.identifier,
                    },
                    item.mut_data().unwrap(),
                ));
            }
        }
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn default_idmap_is_empty() {
        let map: IdMap<i32> = Default::default();
        assert_eq!(map.len(), 0);
    }

    #[test]
    fn initial_length_is_zero() {
        let map = IdMap::<i32>::new();
        assert_eq!(map.len(), 0);
    }

    #[test]
    fn length_increments_on_push() {
        let mut map = IdMap::<i32>::new();
        assert_eq!(map.len(), 0);
        map.push(123);
        assert_eq!(map.len(), 1);
        map.push(0);
        assert_eq!(map.len(), 2);
        map.push(123);
        assert_eq!(map.len(), 3);
    }

    #[test]
    fn push_returns_an_id_that_increments() {
        let mut map = IdMap::<i32>::new();
        assert_eq!(map.push(123).index, 0);
        assert_eq!(map.push(0).index, 1);
        assert_eq!(map.push(123).index, 2);
    }

    #[test]
    fn iter_pushed_elements() {
        let mut map = IdMap::<i32>::new();

        let id_a = map.push(0);
        let id_b = map.push(5325);
        let id_c = map.push(0);

        let mut iter = map.iter();
        assert_eq!(iter.next().unwrap(), (id_a, &0));
        assert_eq!(iter.next().unwrap(), (id_b, &5325));
        assert_eq!(iter.next().unwrap(), (id_c, &0));
    }

    #[test]
    fn iter_after_delete() {
        let mut map = IdMap::<i32>::new();

        map.push(123);
        map.push(5325);
        let id = map.push(1234);
        map.push(123);

        map.delete(id);

        let mut iter = map.iter();
        assert_eq!(*iter.next().unwrap().1, 123);
        assert_eq!(*iter.next().unwrap().1, 5325);
        assert_eq!(*iter.next().unwrap().1, 123);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_mut_pushed_elements() {
        let mut map = IdMap::<i32>::new();

        let id_a = map.push(0);
        let id_b = map.push(5325);
        let id_c = map.push(0);

        let mut iter = map.iter_mut();
        assert_eq!(iter.next().unwrap(), (id_a, &mut 0));
        assert_eq!(iter.next().unwrap(), (id_b, &mut 5325));
        assert_eq!(iter.next().unwrap(), (id_c, &mut 0));
    }

    #[test]
    fn iter_mut_after_delete() {
        let mut map = IdMap::<i32>::new();

        map.push(123);
        map.push(5325);
        let id = map.push(1234);
        map.push(123);

        map.delete(id);

        let mut iter = map.iter_mut();
        assert_eq!(*iter.next().unwrap().1, 123);
        assert_eq!(*iter.next().unwrap().1, 5325);
        assert_eq!(*iter.next().unwrap().1, 123);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn push_after_delete_reuses_id_index() {
        let mut map = IdMap::<i32>::new();
        map.push(12);
        let id = map.push(543);
        map.push(21);

        map.delete(id);
        let new_id = map.push(3213);
        assert_eq!(new_id.index, id.index);
        assert_ne!(new_id.identifier, id.identifier);

        assert!(map.exists(new_id));
        assert!(!map.exists(id));
    }

    #[test]
    fn read_updated_pushed_items() {
        let mut map = IdMap::<i32>::new();

        let id_a = map.push(2);
        let id_b = map.push(5325);
        let id_c = map.push(14);

        map.update(id_b, 2314);

        assert_eq!(*map.ref_data(id_a).unwrap(), 2);
        assert_eq!(*map.ref_data(id_b).unwrap(), 2314);
        assert_eq!(*map.ref_data(id_c).unwrap(), 14);

        assert_eq!(map.len(), 3);
    }

    #[test]
    fn clone_updated_pushed_items() {
        let mut map = IdMap::<i32>::new();

        let id_a = map.push(2);
        let id_b = map.push(5325);
        let id_c = map.push(14);

        map.update(id_b, 2314);

        assert_eq!(map.clone_data(id_a).unwrap(), 2);
        assert_eq!(map.clone_data(id_b).unwrap(), 2314);
        assert_eq!(map.clone_data(id_c).unwrap(), 14);

        assert_eq!(map.len(), 3);
    }

    #[test]
    fn pop_items() {
        let mut map = IdMap::<i32>::new();
        let id_a = map.push(12);
        let id_b = map.push(543);
        let id_c = map.push(21);

        assert_eq!(
            map.pop(Id {
                index: id_a.index,
                identifier: 1000,
            }),
            None
        );
        assert_eq!(3, map.len());

        assert_eq!(map.pop(id_b), Some(543));
        assert_eq!(2, map.len());

        assert_eq!(map.pop(id_a), Some(12));
        assert_eq!(1, map.len());

        assert_eq!(map.pop(id_c), Some(21));
        assert_eq!(0, map.len());
    }

    #[test]
    fn complete_operation() {
        let mut map = IdMap::<i32>::new();

        let id_0 = map.push(12);
        let id_1 = map.push(543);
        let id_2 = map.push(21);

        assert_eq!(
            vec![(id_0, &12), (id_1, &543), (id_2, &21)],
            map.iter().collect::<Vec<_>>()
        );

        map.update(id_1, 132);

        assert_eq!(
            vec![(id_0, &12), (id_1, &132), (id_2, &21)],
            map.iter().collect::<Vec<_>>()
        );

        let id_3 = map.push(41);

        assert_eq!(
            vec![(id_0, &12), (id_1, &132), (id_2, &21), (id_3, &41)],
            map.iter().collect::<Vec<_>>()
        );

        map.delete(id_2);

        assert_eq!(
            vec![(id_0, &12), (id_1, &132), (id_3, &41)],
            map.iter().collect::<Vec<_>>()
        );
    }

    #[test]
    fn exists() {
        let mut map = IdMap::<i32>::new();

        let id_0 = map.push(12);
        let id_1 = map.push(543);
        let id_2 = map.push(21);

        assert!(map.exists(id_0));
        assert!(map.exists(id_1));
        assert!(map.exists(id_2));

        map.delete(id_0);
        assert!(!map.exists(id_0));
    }
}
