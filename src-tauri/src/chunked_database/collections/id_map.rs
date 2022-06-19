use serde::{Deserialize, Serialize};
use std::slice::{Iter, IterMut};

pub type Id = usize;

#[derive(Debug, Serialize, Deserialize)]
pub struct IdMap<T: Serialize> {
    data: Vec<Option<T>>,
    empty_ids: Vec<Id>,
}

impl<T: Serialize> Default for IdMap<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Serialize> IdMap<T> {
    pub fn new() -> Self {
        Self {
            data: vec![],
            empty_ids: vec![],
        }
    }

    pub fn len(&self) -> usize {
        self.data.len() - self.empty_ids.len()
    }

    pub fn push(&mut self, item: T) -> Id {
        if let Some(reused_id) = self.empty_ids.pop() {
            self.data[reused_id] = Some(item);
            return reused_id;
        }
        self.data.push(Some(item));
        self.data.len() - 1
    }

    pub fn pop(&mut self, id: Id) -> Option<T> {
        if self.data.len() < id {
            return None;
        }
        self.empty_ids.push(id);
        self.data[id].take()
    }

    pub fn delete(&mut self, id: Id) {
        self.data[id] = None;
        self.empty_ids.push(id);
    }

    pub fn iter(&self) -> IdMapIter<T> {
        IdMapIter {
            data_iter: self.data.iter(),
            id: 0,
        }
    }

    pub fn iter_mut(&mut self) -> IdMapIterMut<T> {
        IdMapIterMut {
            data_iter: self.data.iter_mut(),
            id: 0,
        }
    }

    pub fn update(&mut self, id: Id, item: T) {
        self.data[id] = Some(item)
    }

    pub fn read(&self, id: Id) -> &Option<T> {
        &self.data[id]
    }
}

pub struct IdMapIter<'a, T> {
    data_iter: Iter<'a, Option<T>>,
    id: Id,
}

impl<'a, T> Iterator for IdMapIter<'a, T> {
    type Item = (Id, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(next) = self.data_iter.next() {
            self.id += 1;
            if let Some(item) = next {
                return Some((self.id - 1, item));
            }
        }
        None
    }
}

pub struct IdMapIterMut<'a, T> {
    data_iter: IterMut<'a, Option<T>>,
    id: Id,
}

impl<'a, T> Iterator for IdMapIterMut<'a, T> {
    type Item = (Id, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(next) = self.data_iter.next() {
            if let Some(item) = next {
                return Some((self.id, item));
            }
            self.id += 1;
        }
        None
    }
}

#[cfg(test)]
mod test {
    use super::IdMap;

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
        assert_eq!(map.push(123), 0);
        assert_eq!(map.push(0), 1);
        assert_eq!(map.push(123), 2);
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

        map.push(0);
        map.push(5325);
        map.push(0);

        let mut iter = map.iter_mut();
        assert_eq!(*iter.next().unwrap().1, 0);
        assert_eq!(*iter.next().unwrap().1, 5325);
        assert_eq!(*iter.next().unwrap().1, 0);
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
    fn push_after_delete_reuses_id() {
        let mut map = IdMap::<i32>::new();
        map.push(12);
        let id = map.push(543);
        map.push(21);

        map.delete(id);
        assert_eq!(map.push(3213), id);
    }

    #[test]
    fn read_updated_pushed_items() {
        let mut map = IdMap::<i32>::new();

        let id_a = map.push(2);
        let id_b = map.push(5325);
        let id_c = map.push(14);

        map.update(id_b, 2314);

        assert_eq!(map.read(id_a).unwrap(), 2);
        assert_eq!(map.read(id_b).unwrap(), 2314);
        assert_eq!(map.read(id_c).unwrap(), 14);

        assert_eq!(map.len(), 3);
    }

    #[test]
    fn pop_items() {
        let mut map = IdMap::<i32>::new();
        let id_a = map.push(12);
        let id_b = map.push(543);
        let id_c = map.push(21);

        assert_eq!(map.pop(100), None);
        assert_eq!(3, map.len());

        assert_eq!(map.pop(id_b), Some(543));
        assert_eq!(2, map.len());

        assert_eq!(map.pop(id_a), Some(12));
        assert_eq!(1, map.len());

        assert_eq!(map.pop(id_c), Some(21));
        assert_eq!(0, map.len());
    }
}
