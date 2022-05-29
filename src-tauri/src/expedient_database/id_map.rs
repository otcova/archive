use std::slice::IterMut;

pub type Id = usize;

pub struct IdMap<T> {
	data: Vec<Option<T>>,
	empty_ids: Vec<Id>,
}

impl<T> IdMap<T> {
    fn new() -> Self {
        Self { data: vec![], empty_ids: vec![] }
    }

    fn len(&self) -> usize {
        self.data.len()
    }
	
	fn push(&mut self, item: T) -> Id {
		if let Some(reused_id) = self.empty_ids.pop() {
			self.data[reused_id] = Some(item);
			return reused_id;
		}
		self.data.push(Some(item));
		self.data.len() - 1
	}
	
	fn delete(&mut self, id: Id) {
		self.data[id] = None;
		self.empty_ids.push(id);
	}
	
	fn iter_mut(&mut self) -> IdMapIterMut<T> {
		IdMapIterMut { data_iter: self.data.iter_mut() }
	}
}

struct IdMapIterMut<'a, T> {
	data_iter: IterMut<'a, Option<T>>
}

impl<'a, T> Iterator for IdMapIterMut<'a, T> {
	type Item = &'a mut T;
	
	fn next(&mut self) -> Option<Self::Item> {
		while let Some(next) = self.data_iter.next() {
			if let Some(item) = next {
				return Some(item);
			}
		}
		None
	}
}


#[cfg(test)]
mod test {
    use super::IdMap;

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
    fn push_returens_an_id_that_increments() {
        let mut map = IdMap::<i32>::new();
        assert_eq!(map.push(123), 0);
        assert_eq!(map.push(0), 1);
        assert_eq!(map.push(123), 2);
    }
	
	#[test]
	fn iter_pushed_elements() {
		let mut map = IdMap::<i32>::new();
		
		map.push(0);
		map.push(5325);
		map.push(0);
				
		let mut iter = map.iter_mut();
		assert_eq!(*iter.next().unwrap(), 0);
		assert_eq!(*iter.next().unwrap(), 5325);
		assert_eq!(*iter.next().unwrap(), 0);
	}
	
	
	#[test]
	fn iter_after_delete() {
		let mut map = IdMap::<i32>::new();
		
		map.push(123);
		map.push(5325);
		let id = map.push(1234);
		map.push(123);
		
		map.delete(id);
		
		let mut iter = map.iter_mut();
		assert_eq!(*iter.next().unwrap(), 123);
		assert_eq!(*iter.next().unwrap(), 5325);
		assert_eq!(*iter.next().unwrap(), 123);
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
}
