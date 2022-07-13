use crate::{observable::*, chunked_database::*};
use std::sync::{Arc, Mutex, RwLock};
use super::Expedient;

pub struct HookPool<'a> {
    observable: Observable<'a, HookContext<'a>>,
    list_observable: Observable<'a, ListHookContext<'a>>,
}

#[derive(Clone)]
struct HookContext<'a> {
    pub database: Arc<RwLock<ChunkedDatabase<Expedient>>>,
    pub expedient_id: Uid,
    pub callback: Arc<Mutex<Box<dyn for<'r> FnMut(Option<&'r Expedient>) + Send + Sync + 'a>>>,
}

#[derive(Clone)]
struct ListHookContext<'a> {
    pub database: Arc<RwLock<ChunkedDatabase<Expedient>>>,
    pub callback: Arc<Mutex<Box<dyn for<'r> FnMut(Vec<(Uid, &'r Expedient)>) + Send + Sync + 'a>>>,
    pub options: ListHookOptions,
}

#[derive(Clone)]
struct ListHookOptions {
    pub sort_by: ListHookOptionsSortBy,
    pub max_list_len: usize,
    pub from_date: i32,
}

#[derive(Clone)]
enum ListHookOptionsSortBy {
    Oldest,
    Newest,
    Similarity(Expedient),
}