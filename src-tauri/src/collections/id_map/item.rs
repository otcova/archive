use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Id {
    pub index: usize,
    pub identifier: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Item<T> {
    pub identifier: usize,
    data: Option<T>,
}

unsafe impl<T: Send> Send for Item<T> {}
unsafe impl<T: Sync> Sync for Item<T> {}

impl<T> Item<T> {
    pub fn new(identifier: usize, data: T) -> Self {
        Self {
            identifier,
            data: Some(data),
        }
    }
    pub fn is_some(&self) -> bool {
        self.data.is_some()
    }
    pub fn is_none(&self) -> bool {
        self.data.is_none()
    }
    pub fn delete(&mut self) {
        self.identifier = 0;
        self.data = None;
    }
    pub fn take(&mut self) -> Option<T> {
        if self.is_some() {
            self.data.take()
        } else {
            None
        }
    }
    pub fn as_ref(&self) -> Option<&T> {
        if self.is_some() {
            self.data.as_ref()
        } else {
            None
        }
    }
    pub fn as_mut(&mut self) -> Option<&mut T> {
        if self.is_some() {
            self.data.as_mut()
        } else {
            None
        }
    }
    pub fn update(&mut self, data: T) {
        self.data = Some(data)
    }
}

impl<T: Clone + Send + Sync> Item<T> {
    pub fn clone_data(&self) -> Option<T> {
        if self.is_some() {
            self.data.clone()
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct RefItem<'a, T> {
    pub id: Id,
    pub data: &'a T,
}

#[derive(Debug)]
pub struct MutItem<'a, T> {
    pub id: Id,
    pub data: &'a mut T,
}
