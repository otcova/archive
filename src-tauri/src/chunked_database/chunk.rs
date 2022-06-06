pub use super::Id;
use super::IdMap;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use crate::database::Database;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DataType<T: Serialize> {
    expedients: IdMap<T>,
}

#[derive(Debug)]
pub struct Chunk<T: Serialize + DeserializeOwned + Default> {
	database: Database<DataType<T>>,
}