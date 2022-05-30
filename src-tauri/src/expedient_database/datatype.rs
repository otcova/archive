use super::{
    collections::{Id, IdMap, DateMap},
    expedient::Expedient,
};

struct RecentDataType {
    expedients: IdMap<Expedient>,
    dates: DateMap<Id>,
}
