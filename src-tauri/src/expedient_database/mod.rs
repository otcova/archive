mod collections;
/// Expedients can be stored in two internal databases:
/// Database<RecentDataType> and Database<AncientDataType>
///
/// In the Database<RecentDataType> are stored the newest used expedients
/// and in the  Database<AncientDataType> are stored all the other ones
mod datatype;
mod expedient;
use collections::*;
use expedient::*;

struct ExpedientDatabase {}

impl ExpedientDatabase {
    fn hook_expedient(&self, _id: Id, _callback: impl FnMut(Expedient) -> bool) {}
    fn hook_expedient_filter(
        &self,
        _filter: Expedient,
        _from: UtcDate,
        _limit: usize,
        _callback: impl FnMut(Vec<Id>) -> bool,
    ) {
    }
    fn hook_all_expedients(
        &self,
        _from: UtcDate,
        _limit: usize,
        _callback: impl FnMut(Vec<Id>) -> bool,
    ) {
    }
    fn hook_all_open_expedients(
        &self,
        _from: UtcDate,
        _limit: usize,
        _callback: impl FnMut(Vec<Id>) -> bool,
    ) {
    }

    fn update_expedient(&self, _id: Id, _expedient: Expedient) {}
    fn create_expedient(&self, _expedient: Expedient) {}
    fn delete_expedient(&self, _id: Id) {}
    fn merge_expedient(&self, _id_a: Id, _id_b: Id) {}
}
