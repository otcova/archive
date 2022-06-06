mod expedient;
use expedient::*;
use crate::chunked_database::*;

struct ExpedientDatabase {
}

impl ExpedientDatabase {
    pub fn hook_expedient(&self, _id: Uid, _callback: impl FnMut(Expedient) -> bool) {}
    pub fn hook_expedient_filter(
        &self,
        _filter: Expedient,
        _from: UtcDate,
        _limit: usize,
        _callback: impl FnMut(Vec<Uid>) -> bool,
    ) {
    }
    pub fn hook_all_expedients(
        &self,
        _from: UtcDate,
        _limit: usize,
        _callback: impl FnMut(Vec<Uid>) -> bool,
    ) {
    }
    pub fn hook_all_open_expedients(
        &self,
        _from: UtcDate,
        _limit: usize,
        _callback: impl FnMut(Vec<Uid>) -> bool,
    ) {
    }

    pub fn update_expedient(&self, _id: Uid, _expedient: Expedient) {}
    pub fn create_expedient(&self, _expedient: Expedient) {}
    pub fn delete_expedient(&self, _id: Uid) {}
    pub fn merge_expedient(&self, _id_a: Uid, _id_b: Uid) {}
}
