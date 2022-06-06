mod chunk;
mod collections;

use chunk::Chunk;
pub use collections::*;
use serde::{Serialize, de::DeserializeOwned};

/// Data is stored in two internal databases:
/// dynamic_db and ancient_db
///
/// In the dynamic_db is stored the newest data.
/// This way, you can skip scaning very old data.
///
/// In the ancient_db are stored all the data considered old.

pub enum Uid {
	DYNAMIC(chunk::Id),
	ANCIENT(chunk::Id),
}

pub struct ChunkedDatabase<T: Serialize + DeserializeOwned + Default> {
    dynamic_db: Chunk<T>,
    ancient_db: Chunk<T>,
}