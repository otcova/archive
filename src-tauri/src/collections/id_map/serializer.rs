use super::*;
use serde::Serialize;

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
