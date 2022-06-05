mod collections;
mod datatype;
mod expedient;
use self::datatype::{ExpedientId, DataType};
use collections::UtcDate;
use expedient::*;
use crate::database::Database;

/// Expedients are stored in two internal databases:
/// dynamic_db and ancient_db
///
/// In the dynamic_db are stored the newest used expedients.
/// When filtering (by default) only these expedients are scaned.
/// 
/// In the ancient_db are stored all the other ones.

struct ExpedientDatabase {
    dynamic_db: Database<DataType>,
    ancient_db: Database<DataType>,
}

impl ExpedientDatabase {
    pub fn hook_expedient(&self, _id: ExpedientId, _callback: impl FnMut(Expedient) -> bool) {}
    pub fn hook_expedient_filter(
        &self,
        _filter: Expedient,
        _from: UtcDate,
        _limit: usize,
        _callback: impl FnMut(Vec<ExpedientId>) -> bool,
    ) {
    }
    pub fn hook_all_expedients(
        &self,
        _from: UtcDate,
        _limit: usize,
        _callback: impl FnMut(Vec<ExpedientId>) -> bool,
    ) {
    }
    pub fn hook_all_open_expedients(
        &self,
        _from: UtcDate,
        _limit: usize,
        _callback: impl FnMut(Vec<ExpedientId>) -> bool,
    ) {
    }

    pub fn update_expedient(&self, _id: ExpedientId, _expedient: Expedient) {}
    pub fn create_expedient(&self, _expedient: Expedient) {}
    pub fn delete_expedient(&self, _id: ExpedientId) {}
    pub fn merge_expedient(&self, _id_a: ExpedientId, _id_b: ExpedientId) {}
}
