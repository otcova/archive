use super::*;
use crate::{chunked_database::*, observable::*};
use std::sync::{Arc, Mutex, RwLock};

#[derive(Default)]
pub struct HookPool<'a> {
    observable: Observable<'a, HookContext<'a>>,
    list_observable: Observable<'a, ListExpedientsHookContext<'a>>,
    list_oreders_observable: Observable<'a, ListOrdersHookContext<'a>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HookId {
    Expedient(Id),
    ListExpedients(Id),
    ListExpedientOrders(Id),
}

// Expedient Hook

#[derive(Clone)]
struct HookContext<'a> {
    pub database: Arc<RwLock<ChunkedDatabase<Expedient>>>,
    pub expedient_id: Uid,
    pub callback: Arc<Mutex<Box<dyn for<'r> FnMut(Option<&'r Expedient>) + Send + Sync + 'a>>>,
}

// List of Expedients Hook

#[derive(Clone)]
struct ListExpedientsHookContext<'a> {
    pub database: Arc<RwLock<ChunkedDatabase<Expedient>>>,
    pub callback:
        Arc<Mutex<Box<dyn for<'r> FnMut(Vec<(Uid, &'r Expedient, f32)>) + Send + Sync + 'a>>>,
    pub options: ListExpedientsHookOptions,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ListExpedientsHookOptions {
    pub filter: Expedient,
    pub max_list_len: usize,
}

// List of Expedient Orders Hook

#[derive(Clone)]
struct ListOrdersHookContext<'a> {
    pub database: Arc<RwLock<ChunkedDatabase<Expedient>>>,
    pub callback:
        Arc<Mutex<Box<dyn for<'r> FnMut(Vec<(Uid, usize, &'r Expedient)>) + Send + Sync + 'a>>>,
    pub options: ListOrdersHookOptions,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct ListOrdersHookOptions {
    pub filter: Option<Expedient>,
    pub sort_by: ListOrdersHookOptionsSortBy,
    pub max_list_len: usize,
    pub from_date: i32,
    pub show_todo: bool,
    pub show_urgent: bool,
    pub show_pending: bool,
    pub show_done: bool,
}

#[derive(Clone, Deserialize, Serialize)]
pub enum ListOrdersHookOptionsSortBy {
    Oldest,
    Newest,
}

impl<'a> ExpedientDatabase<'a> {
    pub fn release_hook(&mut self, hook_id: HookId) {
        match hook_id {
            HookId::Expedient(id) => self.hook_pool.observable.unsubscrive(id),
            HookId::ListExpedients(id) => self.hook_pool.list_observable.unsubscrive(id),
            HookId::ListExpedientOrders(id) => {
                self.hook_pool.list_oreders_observable.unsubscrive(id)
            }
        }
    }
    pub fn release_all_hooks(&mut self) {
        self.hook_pool = Default::default();
    }
    pub fn dispath_change(&mut self) {
        self.hook_pool.observable.trigger();
        self.hook_pool.list_observable.async_trigger();
        self.hook_pool.list_oreders_observable.async_trigger();
    }

    pub fn hook_expedient(
        &mut self,
        id: Uid,
        callback: impl for<'r> FnMut(Option<&'r Expedient>) -> () + Send + Sync + 'a,
    ) -> HookId {
        HookId::Expedient(self.hook_pool.observable.subscrive(
            Callback {
                callback: |context| {
                    let database = context.database.read().unwrap();
                    let expedient = database.read(context.expedient_id);
                    (context.callback.lock().unwrap())(expedient)
                },
                context: HookContext {
                    database: self.database.clone(),
                    expedient_id: id,
                    callback: Arc::new(Mutex::new(Box::new(callback))),
                },
            },
            InstantTriggerType::Sync,
        ))
    }

    pub fn hook_list_oreders(
        &mut self,
        options: ListOrdersHookOptions,
        callback: impl for<'r> FnMut(Vec<(Uid, usize, &'r Expedient)>) -> () + Send + Sync + 'a,
    ) -> HookId {
        HookId::ListExpedientOrders(self.hook_pool.list_oreders_observable.subscrive(
            Callback {
                callback: |context| {
                    let database = context.database.read().unwrap();

                    let mut list_orders: Vec<_> = database
                        .iter()
                        .flat_map(|(id, exp)| {
                            (0..exp.orders.len()).map(move |index| (id, index, exp))
                        })
                        .filter(|(_, index, expedient)| {
                            let order = &expedient.orders[*index];
                            order.date.date_hash() <= context.options.from_date
                                && match order.state {
                                    OrderState::Done => context.options.show_done,
                                    OrderState::Todo => context.options.show_todo,
                                    OrderState::Urgent => context.options.show_urgent,
                                    OrderState::Pending => context.options.show_pending,
                                }
                        })
                        .collect();

                    match context.options.sort_by {
                        ListOrdersHookOptionsSortBy::Newest => {
                            list_orders.sort_unstable_by_key(|(_, index, expedient)| {
                                -expedient.orders[*index].date.date_hash()
                            })
                        }
                        ListOrdersHookOptionsSortBy::Oldest => {
                            list_orders.sort_unstable_by_key(|(_, index, expedient)| {
                                expedient.orders[*index].date.date_hash()
                            })
                        }
                    };
                    list_orders.truncate(context.options.max_list_len);
                    (context.callback.lock().unwrap())(list_orders);

                    // TODO: check on ancient database
                },
                context: ListOrdersHookContext {
                    database: self.database.clone(),
                    options,
                    callback: Arc::new(Mutex::new(Box::new(callback))),
                },
            },
            InstantTriggerType::Async,
        ))
    }

    pub fn hook_list_expedients(
        &mut self,
        options: ListExpedientsHookOptions,
        callback: impl for<'r> FnMut(Vec<(Uid, &'r Expedient, f32)>) -> () + Send + Sync + 'a,
    ) -> HookId {
        HookId::ListExpedients(self.hook_pool.list_observable.subscrive(
            Callback {
                callback: |context| {
                    let database = context.database.read().unwrap();

                    let mut list: Vec<_> = database
                        .iter()
                        .map(|(id, expedient)| {
                            (id, expedient, expedient.similarity(&context.options.filter))
                        })
                        .filter(|(_, _, similarity)| *similarity > 0.)
                        .collect();

                    // Sort by similarity
                    list.sort_unstable_by(|(_, _, a), (_, _, b)| {
                        b.partial_cmp(a)
                            .expect("Partial compare of f32 is None, data can not be sorted")
                    });

                    list.truncate(context.options.max_list_len);
                    (context.callback.lock().unwrap())(list);

                    // TODO: check on ancient database
                },
                context: ListExpedientsHookContext {
                    database: self.database.clone(),
                    options,
                    callback: Arc::new(Mutex::new(Box::new(callback))),
                },
            },
            InstantTriggerType::Async,
        ))
    }
}
