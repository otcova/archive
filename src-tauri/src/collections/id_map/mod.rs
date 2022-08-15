mod item;
mod serializer;
pub use item::Id;
use item::*;
use serde::Deserialize;
pub use serializer::*;
use std::slice::{Iter, IterMut};

#[derive(Debug, Deserialize)]
pub struct IdMap<T> {
    data: Vec<Item<T>>,
    empty_indexes: Vec<usize>,
    last_identifier: usize,
}

unsafe impl<T: Send> Send for IdMap<T> {}
unsafe impl<T: Sync> Sync for IdMap<T> {}

impl<T> Default for IdMap<T> {
    fn default() -> Self {
        Self {
            data: vec![],
            empty_indexes: vec![],
            last_identifier: 0,
        }
    }
}

impl<T> IdMap<T> {
    pub fn len(&self) -> usize {
        self.data.len() - self.empty_indexes.len()
    }

    pub fn push(&mut self, item: T) -> Id {
        self.last_identifier += 1;
        if let Some(reused_id) = self.empty_indexes.pop() {
            self.data[reused_id] = Item::new(self.last_identifier, item);
            Id {
                index: reused_id,
                identifier: self.last_identifier,
            }
        } else {
            self.data.push(Item::new(self.last_identifier, item));
            Id {
                index: self.data.len() - 1,
                identifier: self.last_identifier,
            }
        }
    }

    pub fn take(&mut self, id: Id) -> Option<T> {
        if self.exists(id) {
            self.empty_indexes.push(id.index);
            self.data[id.index].take()
        } else {
            None
        }
    }

    pub fn delete(&mut self, id: Id) {
        self.take(id);
    }

    pub fn filter(&mut self, mut filter: impl FnMut(MutItem<T>) -> bool) {
        for index in 0..self.data.len() {
            let id = Id {
                index,
                identifier: self.data[index].identifier,
            };
            if let Some(data) = self.data[index].as_mut() {
                if !filter(MutItem { id, data }) {
                    self.delete(id);
                }
            }
        }
    }

    pub fn exists(&self, id: Id) -> bool {
        if id.index < self.data.len() {
            self.data[id.index].identifier == id.identifier
        } else {
            false
        }
    }

    pub fn get_ref(&self, id: Id) -> Option<&T> {
        if self.exists(id) {
            self.data[id.index].as_ref()
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, id: Id) -> Option<&mut T> {
        if self.exists(id) {
            self.data[id.index].as_mut()
        } else {
            None
        }
    }

    pub fn update(&mut self, id: Id, new_data: T) {
        if let Some(data_mut) = self.get_mut(id) {
            *data_mut = new_data
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
}

pub struct IdMapIter<'a, T> {
    data_iter: Iter<'a, Item<T>>,
    index: usize,
}

impl<'a, T> Iterator for IdMapIter<'a, T> {
    type Item = RefItem<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(item) = self.data_iter.next() {
            self.index += 1;
            if item.is_some() {
                return Some(RefItem {
                    id: Id {
                        index: self.index - 1,
                        identifier: item.identifier,
                    },
                    data: item.as_ref().unwrap(),
                });
            }
        }
        None
    }
}

pub struct IdMapIterMut<'a, T> {
    data_iter: IterMut<'a, Item<T>>,
    index: usize,
}

impl<'a, T> Iterator for IdMapIterMut<'a, T> {
    type Item = MutItem<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(item) = self.data_iter.next() {
            self.index += 1;
            if item.is_some() {
                return Some(MutItem {
                    id: Id {
                        index: self.index - 1,
                        identifier: item.identifier,
                    },
                    data: item.as_mut().unwrap(),
                });
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
        let map = IdMap::<i32>::default();
        assert_eq!(map.len(), 0);
    }

    #[test]
    fn length_increments_on_push() {
        let mut map = IdMap::<i32>::default();
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
        let mut map = IdMap::<i32>::default();
        assert_eq!(map.push(123).index, 0);
        assert_eq!(map.push(0).index, 1);
        assert_eq!(map.push(123).index, 2);
    }

    #[test]
    fn iter_pushed_elements() {
        let mut map = IdMap::<i32>::default();

        let id_a = map.push(0);
        let id_b = map.push(5325);
        let id_c = map.push(-1);

        let mut iter = map.iter();

        let a = iter.next().unwrap();
        assert_eq!(a.id, id_a);
        assert_eq!(*a.data, 0);

        let b = iter.next().unwrap();
        assert_eq!(b.id, id_b);
        assert_eq!(*b.data, 5325);

        let c = iter.next().unwrap();
        assert_eq!(c.id, id_c);
        assert_eq!(*c.data, -1);

        assert!(iter.next().is_none());
    }

    #[test]
    fn iter_after_delete() {
        let mut map = IdMap::<i32>::default();

        map.push(123);
        map.push(5325);
        let id = map.push(1234);
        map.push(123);

        map.delete(id);

        let mut iter = map.iter();
        assert_eq!(*iter.next().unwrap().data, 123);
        assert_eq!(*iter.next().unwrap().data, 5325);
        assert_eq!(*iter.next().unwrap().data, 123);
        assert!(iter.next().is_none());
    }

    #[test]
    fn iter_mut_pushed_elements() {
        let mut map = IdMap::<i32>::default();

        let id_a = map.push(-194984);
        let id_b = map.push(9491);
        let id_c = map.push(0);

        let mut iter = map.iter_mut();
        let a = iter.next().unwrap();
        assert_eq!(a.id, id_a);
        assert_eq!(*a.data, -194984);

        let b = iter.next().unwrap();
        assert_eq!(b.id, id_b);
        assert_eq!(*b.data, 9491);

        let c = iter.next().unwrap();
        assert_eq!(c.id, id_c);
        assert_eq!(*c.data, 0);

        assert!(iter.next().is_none());
    }

    #[test]
    fn iter_mut_after_delete() {
        let mut map = IdMap::<i32>::default();

        map.push(123);
        map.push(5325);
        let id = map.push(1234);
        map.push(123);

        map.delete(id);

        let mut iter = map.iter_mut();
        assert_eq!(*iter.next().unwrap().data, 123);
        assert_eq!(*iter.next().unwrap().data, 5325);
        assert_eq!(*iter.next().unwrap().data, 123);
        assert!(iter.next().is_none());
    }

    #[test]
    fn push_after_delete_reuses_id_index() {
        let mut map = IdMap::<i32>::default();
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
        let mut map = IdMap::<i32>::default();

        let id_a = map.push(2);
        let id_b = map.push(5325);
        let id_c = map.push(14);

        map.update(id_b, 2314);

        assert_eq!(*map.get_ref(id_a).unwrap(), 2);
        assert_eq!(*map.get_ref(id_b).unwrap(), 2314);
        assert_eq!(*map.get_ref(id_c).unwrap(), 14);

        assert_eq!(map.len(), 3);
    }

    #[test]
    fn pop_items() {
        let mut map = IdMap::<i32>::default();
        let id_a = map.push(12);
        let id_b = map.push(543);
        let id_c = map.push(21);

        assert_eq!(
            map.take(Id {
                index: id_a.index,
                identifier: 1000,
            }),
            None
        );
        assert_eq!(3, map.len());

        assert_eq!(map.take(id_b), Some(543));
        assert_eq!(2, map.len());

        assert_eq!(map.take(id_a), Some(12));
        assert_eq!(1, map.len());

        assert_eq!(map.take(id_c), Some(21));
        assert_eq!(0, map.len());
    }

    #[test]
    fn complete_operation() {
        let mut map = IdMap::<i32>::default();

        let id_0 = map.push(12);
        let id_1 = map.push(543);
        let id_2 = map.push(21);

        assert_eq!(
            vec![(id_0, 12), (id_1, 543), (id_2, 21)],
            map.iter()
                .map(|item| (item.id, *item.data))
                .collect::<Vec<_>>()
        );

        map.update(id_1, 132);

        assert_eq!(
            vec![(id_0, 12), (id_1, 132), (id_2, 21)],
            map.iter()
                .map(|item| (item.id, *item.data))
                .collect::<Vec<_>>()
        );

        let id_3 = map.push(41);

        assert_eq!(
            vec![(id_0, 12), (id_1, 132), (id_2, 21), (id_3, 41)],
            map.iter()
                .map(|item| (item.id, *item.data))
                .collect::<Vec<_>>()
        );

        map.delete(id_2);

        assert_eq!(
            vec![(id_0, 12), (id_1, 132), (id_3, 41)],
            map.iter()
                .map(|item| (item.id, *item.data))
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn exists_before_and_after_delete() {
        let mut map = IdMap::<i32>::default();

        let id_0 = map.push(12);
        let id_1 = map.push(543);
        let id_2 = map.push(21);

        assert!(map.exists(id_0));
        assert!(map.exists(id_1));
        assert!(map.exists(id_2));

        map.delete(id_0);
        assert!(!map.exists(id_0));
    }

    #[test]
    fn exists_before_and_after_take() {
        let mut map = IdMap::<i32>::default();

        let id_0 = map.push(12);
        let id_1 = map.push(543);
        let id_2 = map.push(21);

        assert!(map.exists(id_0));
        assert!(map.exists(id_1));
        assert!(map.exists(id_2));

        map.take(id_0);
        assert!(!map.exists(id_0));
    }
}
