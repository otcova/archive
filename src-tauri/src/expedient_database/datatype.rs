use super::{
    collections::{Id, IdMap},
    expedient::Expedient,
};
use serde::{Deserialize, Serialize};

pub type ExpedientId = Id;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DataType {
    expedients: IdMap<Expedient>,
}
