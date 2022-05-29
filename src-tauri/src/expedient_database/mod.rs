mod expedient;
mod id_map;
use expedient::*;

struct ExpedientDatabase {}

impl ExpedientDatabase {
    fn hook_expedient(&self, _id: id_map::Id, _callback: impl FnMut(Expedient) -> bool) {}
    fn hook_expedient_filter(
        &self,
        _filter: Expedient,
        _from: UtcDate,
        _limit: usize,
        _callback: impl FnMut(Vec<id_map::Id>) -> bool,
    ) {
    }
    fn hook_all_expedients(
        &self,
        _from: UtcDate,
        _limit: usize,
        _callback: impl FnMut(Vec<id_map::Id>) -> bool,
    ) {
    }
    fn hook_all_open_expedients(
        &self,
        _from: UtcDate,
        _limit: usize,
        _callback: impl FnMut(Vec<id_map::Id>) -> bool,
    ) {
    }

    fn update_expedient(&self, _id: id_map::Id, _expedient: Expedient) {}
    fn create_expedient(&self, _expedient: Expedient) {}
    fn delete_expedient(&self, _id: id_map::Id) {}
    fn merge_expedient(&self, _id_a: id_map::Id, _id_b: id_map::Id) {}
}
