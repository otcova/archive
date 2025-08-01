use super::filter::Filter;
use super::*;
use crate::{chunked_database::*, observable::*};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex, RwLock},
};

#[derive(Default)]
pub struct HookPool<'a> {
    observable: Observable<HookContext<'a>>,
    list_observable: AsyncObservable<'a, ListExpedientsHookContext<'a>>,
    list_orders_observable: AsyncObservable<'a, ListOrdersHookContext<'a>>,
    list_filter: AsyncObservable<'a, ListFilterHookContext<'a>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HookId {
    Expedient(Id),
    ListExpedients(Id),
    ListExpedientOrders(Id),
    ListFilter(Id),
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
        Arc<Mutex<Box<dyn for<'r> FnMut(&Vec<(Uid, &'r Expedient, f32)>) + Send + Sync + 'a>>>,
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
        Arc<Mutex<Box<dyn for<'r> FnMut(&Vec<(Uid, usize, &'r Expedient)>) + Send + Sync + 'a>>>,
    pub options: ListOrdersHookOptions,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListOrdersHookFilter {
    pub car_code: String,
    pub user: String,
    pub body: String,
    pub popularity: i32,
}

impl ListOrdersHookFilter {
    fn to_lowercase(&mut self) {
        self.car_code = self.car_code.to_lowercase();
        self.user = self.user.to_lowercase();
        self.body = self.body.to_lowercase();
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListOrdersHookOptions {
    pub filter: Option<ListOrdersHookFilter>,
    pub sort_by: ListOrdersHookOptionsSortBy,
    pub max_list_len: usize,
    pub from_date: UtcDate,
    pub show_urgent: bool,
    pub show_todo: bool,
    pub show_awaiting: bool,
    pub show_instore: bool,
    pub show_done: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ListOrdersHookOptionsSortBy {
    Oldest,
    Newest,
}

#[derive(Clone)]
struct ListFilterHookContext<'a> {
    pub database: Arc<RwLock<ChunkedDatabase<Expedient>>>,
    pub callback: Arc<Mutex<Box<dyn for<'r> FnMut(&Vec<&str>) + Send + Sync + 'a>>>,
    pub filter: String,
}

impl<'a> ExpedientDatabase<'a> {
    pub fn release_hook(&mut self, hook_id: HookId) {
        match hook_id {
            HookId::Expedient(id) => self.hook_pool.observable.unsubscrive(id),
            HookId::ListExpedients(id) => self.hook_pool.list_observable.unsubscrive(id),
            HookId::ListExpedientOrders(id) => {
                self.hook_pool.list_orders_observable.unsubscrive(id)
            }
            HookId::ListFilter(id) => self.hook_pool.list_filter.unsubscrive(id),
        }
    }
    pub fn release_all_hooks(&mut self) {
        self.hook_pool = Default::default();
    }
    pub fn interrupt_dispatch(&mut self) {
        self.hook_pool.list_observable.stop_trigger();
        self.hook_pool.list_orders_observable.stop_trigger();
    }
    pub fn dispatch_change(&mut self) {
        self.hook_pool.observable.trigger();
        self.hook_pool.list_observable.trigger();
        self.hook_pool.list_orders_observable.trigger();
    }

    pub fn hook_expedient(
        &mut self,
        id: Uid,
        callback: impl for<'r> FnMut(Option<&'r Expedient>) -> () + Send + Sync + 'a,
    ) -> HookId {
        HookId::Expedient(self.hook_pool.observable.subscrive(
            Callback::new(
                HookContext {
                    database: self.database.clone(),
                    expedient_id: id,
                    callback: Arc::new(Mutex::new(Box::new(callback))),
                },
                |context| {
                    let database = context.database.read().unwrap();
                    let expedient = database.read(context.expedient_id);
                    (context.callback.lock().unwrap())(expedient);
                },
            ),
            true,
        ))
    }

    fn list_orders<'b>(
        options: &ListOrdersHookOptions,
        expedients: impl Iterator<Item = (Uid, &'b Expedient)>,
        concat_with: &mut Vec<(Uid, usize, &'b Expedient)>,
        process: &AsyncCallbackProcess,
        user_occurrences: &HashMap<String, i32>,
    ) -> Option<Vec<(Uid, usize, &'b Expedient)>> {
        let mut filtered_expedients: Box<dyn Iterator<Item = _>> = Box::new(expedients);
        process.terminate_if_requested()?;

        if let Some(ref filter) = options.filter {
            let car_code_filter = Filter::new(&filter.car_code.replace("_", "").replace(" ", ""));

            if filter.car_code != "" {
                filtered_expedients = Box::new(filtered_expedients.filter(move |(_, exp)| {
                    car_code_filter.test(&exp.license_plate.replace("_", "").replace(" ", "")) > 0
                        || car_code_filter.test(&exp.vin.replace("_", "").replace(" ", "")) > 0
                }))
            }
        }

        process.terminate_if_requested()?;

        if let Some(ref filter) = options.filter {
            let user_filter = Filter::new(&filter.user);

            if filter.user != "" {
                filtered_expedients = Box::new(
                    filtered_expedients.filter(move |(_, exp)| user_filter.test(&exp.user) > 0),
                )
            }
        }

        process.terminate_if_requested()?;

        let mut orders: Box<dyn Iterator<Item = _>> = Box::new(
            filtered_expedients
                .flat_map(|(id, exp)| (0..exp.orders.len()).map(move |index| (id, index, exp)))
                .filter(|(_, index, expedient)| {
                    let order = &expedient.orders[*index];
                    order.date.date_hash() <= options.from_date.date_hash()
                        && match order.state {
                            OrderState::Urgent => options.show_urgent,
                            OrderState::Todo => options.show_todo,
                            OrderState::Awaiting => options.show_awaiting,
                            OrderState::InStore => options.show_instore,
                            OrderState::Done => options.show_done,
                        }
                }),
        );

        process.terminate_if_requested()?;

        if let Some(ref filter) = options.filter {
            let body_filter = Filter::new(&filter.body);

            if filter.body != "" {
                orders = Box::new(orders.filter(move |(_, index, expedient)| {
                    body_filter.test(expedient.model.as_str()) > 0
                        || body_filter.test(&expedient.description) > 0
                        || body_filter.test(&expedient.orders[*index].title) > 0
                        || body_filter.test(&expedient.orders[*index].description) > 0
                }));
                process.terminate_if_requested()?;
            }
        }

        let mut list_orders: Vec<_> = orders.collect();
        process.terminate_if_requested()?;

        list_orders.append(concat_with);
        process.terminate_if_requested()?;

        if let Some(ref filter) = options.filter {
            if filter.popularity != 0 {
                list_orders.retain(|(_, _, expedient)| {
                    let mut username = expedient.user.to_lowercase();
                    username.remove_matches(" ");
                    let occurrances = user_occurrences[&username];
                    if filter.popularity > 0 {
                        occurrances >= filter.popularity
                    } else {
                        occurrances <= -filter.popularity
                    }
                });
                process.terminate_if_requested()?;
            }
        }

        match options.sort_by {
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

        process.terminate_if_requested()?;

        list_orders.truncate(options.max_list_len);

        process.terminate_if_requested()?;

        Some(list_orders)
    }

    pub fn hook_list_orders(
        &mut self,
        mut options: ListOrdersHookOptions,
        callback: impl for<'r> FnMut(&Vec<(Uid, usize, &'r Expedient)>) -> () + Send + Sync + 'a,
    ) -> HookId {
        if let Some(ref mut filter) = options.filter {
            filter.to_lowercase();
        }

        HookId::ListExpedientOrders(self.hook_pool.list_orders_observable.subscrive(
            AsyncCallback::new(
                ListOrdersHookContext {
                    database: self.database.clone(),
                    options,
                    callback: Arc::new(Mutex::new(Box::new(callback))),
                },
                |context, process| {
                    let database = context.database.read().unwrap();

                    // This way of counting user_occurrences can be improved
                    let mut user_occurrences = HashMap::<String, i32>::new();
                    if let Some(ref filter) = context.options.filter {
                        if filter.popularity != 0 {
                            for (_, expedient) in database.iter_all() {
                                let mut username = expedient.user.to_lowercase();
                                username.remove_matches(" ");
                                *user_occurrences.entry(username).or_insert(1) += 1;
                            }
                            process.terminate_if_requested()?;
                        }
                    }

                    let mut dynamic_list = Self::list_orders(
                        &context.options,
                        database.iter(),
                        &mut vec![],
                        &process,
                        &user_occurrences,
                    )?;
                    (context.callback.lock().unwrap())(&dynamic_list);

                    let full_list = Self::list_orders(
                        &context.options,
                        database.iter_ancient(),
                        &mut dynamic_list,
                        &process,
                        &user_occurrences,
                    )?;
                    (context.callback.lock().unwrap())(&full_list);

                    Some(())
                },
            ),
            true,
        ))
    }

    pub fn hook_list_expedients(
        &mut self,
        options: ListExpedientsHookOptions,
        callback: impl for<'r> FnMut(&Vec<(Uid, &'r Expedient, f32)>) -> () + Send + Sync + 'a,
    ) -> HookId {
        HookId::ListExpedients(self.hook_pool.list_observable.subscrive(
            AsyncCallback::new(
                ListExpedientsHookContext {
                    database: self.database.clone(),
                    options,
                    callback: Arc::new(Mutex::new(Box::new(callback))),
                },
                |context, process| {
                    let database = context.database.read().unwrap();

                    let mut list: Vec<_> = database
                        .iter()
                        .map(|(id, expedient)| {
                            (id, expedient, expedient.similarity(&context.options.filter))
                        })
                        .filter(|(_, _, similarity)| *similarity > 0.)
                        .collect();

                    process.terminate_if_requested()?;

                    // Sort by similarity
                    list.sort_unstable_by(|(_, _, a), (_, _, b)| {
                        b.partial_cmp(a)
                            .expect("Partial compare of f32 is None, data can not be sorted")
                    });

                    list.truncate(context.options.max_list_len);

                    process.terminate_if_requested()?;

                    (context.callback.lock().unwrap())(&list);

                    // TODO: check on ancient database

                    Some(())
                },
            ),
            true,
        ))
    }

    fn list_filter<'b>(
        filter: &Filter,
        expedients: impl Iterator<Item = (&'b str, UtcDate)>,
        concat_with: &mut Vec<(&'b str, UtcDate)>,
        process: &AsyncCallbackProcess,
    ) -> Option<Vec<(&'b str, UtcDate)>> {
        const MAX_LIST_LEN: usize = 5;

        process.terminate_if_requested()?;

        let mut list: Vec<_> = expedients
            .map(|(data, date)| (data.trim(), date))
            .filter(|(data, _)| !data.is_empty() && filter.test(data) > 0)
            .collect();

        process.terminate_if_requested()?;

        list.append(concat_with);

        process.terminate_if_requested()?;

        // Eliminate duplicates
        list.sort_unstable_by_key(|(text, _)| *text);
        list.dedup_by_key(|(text, _)| *text);

        process.terminate_if_requested()?;

        list.sort_unstable_by_key(|(data, date)| (-(filter.test(data) as i32), -date.date_hash()));
        list.truncate(MAX_LIST_LEN);

        process.terminate_if_requested()?;

        Some(list)
    }

    pub fn hook_list_users(
        &mut self,
        filter: String,
        callback: impl for<'r> FnMut(&Vec<&str>) -> () + Send + Sync + 'a,
    ) -> HookId {
        HookId::ListFilter(self.hook_pool.list_filter.subscrive(
            AsyncCallback::new(
                ListFilterHookContext {
                    database: self.database.clone(),
                    filter,
                    callback: Arc::new(Mutex::new(Box::new(callback))),
                },
                |context, process| {
                    let user_filter = Filter::new(&context.filter);
                    let database = context.database.read().unwrap();

                    let mut dynamic_list = Self::list_filter(
                        &user_filter,
                        database.iter().map(|(_, expedient)| {
                            (expedient.user.as_str(), expedient.newest_date())
                        }),
                        &mut vec![],
                        &process,
                    )?;
                    (context.callback.lock().unwrap())(
                        &dynamic_list.iter().map(|(data, _)| *data).collect(),
                    );

                    let full_list = Self::list_filter(
                        &user_filter,
                        database.iter_ancient().map(|(_, expedient)| {
                            (expedient.user.as_str(), expedient.newest_date())
                        }),
                        &mut dynamic_list,
                        &process,
                    )?;
                    (context.callback.lock().unwrap())(
                        &full_list.iter().map(|(data, _)| *data).collect(),
                    );

                    Some(())
                },
            ),
            true,
        ))
    }

    pub fn hook_list_models(
        &mut self,
        filter: String,
        callback: impl for<'r> FnMut(&Vec<&str>) -> () + Send + Sync + 'a,
    ) -> HookId {
        HookId::ListFilter(self.hook_pool.list_filter.subscrive(
            AsyncCallback::new(
                ListFilterHookContext {
                    database: self.database.clone(),
                    filter,
                    callback: Arc::new(Mutex::new(Box::new(callback))),
                },
                |context, process| {
                    let model_filter = Filter::new(&context.filter);
                    let database = context.database.read().unwrap();

                    let mut dynamic_list = Self::list_filter(
                        &model_filter,
                        database.iter().map(|(_, expedient)| {
                            (expedient.model.as_str(), expedient.newest_date())
                        }),
                        &mut vec![],
                        &process,
                    )?;
                    (context.callback.lock().unwrap())(
                        &dynamic_list.iter().map(|(data, _)| *data).collect(),
                    );

                    let full_list = Self::list_filter(
                        &model_filter,
                        database.iter_ancient().map(|(_, expedient)| {
                            (expedient.model.as_str(), expedient.newest_date())
                        }),
                        &mut dynamic_list,
                        &process,
                    )?;
                    (context.callback.lock().unwrap())(
                        &full_list.iter().map(|(data, _)| *data).collect(),
                    );

                    Some(())
                },
            ),
            true,
        ))
    }

    pub fn hook_list_license_plates(
        &mut self,
        filter: String,
        callback: impl for<'r> FnMut(&Vec<&str>) -> () + Send + Sync + 'a,
    ) -> HookId {
        HookId::ListFilter(self.hook_pool.list_filter.subscrive(
            AsyncCallback::new(
                ListFilterHookContext {
                    database: self.database.clone(),
                    filter,
                    callback: Arc::new(Mutex::new(Box::new(callback))),
                },
                |context, process| {
                    let license_plate_filter = Filter::new(&context.filter);
                    let database = context.database.read().unwrap();

                    let mut dynamic_list = Self::list_filter(
                        &license_plate_filter,
                        database.iter().map(|(_, expedient)| {
                            (expedient.license_plate.as_str(), expedient.newest_date())
                        }),
                        &mut vec![],
                        &process,
                    )?;
                    (context.callback.lock().unwrap())(
                        &dynamic_list.iter().map(|(data, _)| *data).collect(),
                    );

                    let full_list = Self::list_filter(
                        &license_plate_filter,
                        database.iter_ancient().map(|(_, expedient)| {
                            (expedient.license_plate.as_str(), expedient.newest_date())
                        }),
                        &mut dynamic_list,
                        &process,
                    )?;
                    (context.callback.lock().unwrap())(
                        &full_list.iter().map(|(data, _)| *data).collect(),
                    );

                    Some(())
                },
            ),
            true,
        ))
    }

    pub fn hook_list_vins(
        &mut self,
        filter: String,
        callback: impl for<'r> FnMut(&Vec<&str>) -> () + Send + Sync + 'a,
    ) -> HookId {
        HookId::ListFilter(self.hook_pool.list_filter.subscrive(
            AsyncCallback::new(
                ListFilterHookContext {
                    database: self.database.clone(),
                    filter,
                    callback: Arc::new(Mutex::new(Box::new(callback))),
                },
                |context, process| {
                    let vin_filter = Filter::new(&context.filter);
                    let database = context.database.read().unwrap();

                    let mut dynamic_list = Self::list_filter(
                        &vin_filter,
                        database.iter().map(|(_, expedient)| {
                            (expedient.vin.as_str(), expedient.newest_date())
                        }),
                        &mut vec![],
                        &process,
                    )?;
                    (context.callback.lock().unwrap())(
                        &dynamic_list.iter().map(|(data, _)| *data).collect(),
                    );

                    let full_list = Self::list_filter(
                        &vin_filter,
                        database.iter_ancient().map(|(_, expedient)| {
                            (expedient.vin.as_str(), expedient.newest_date())
                        }),
                        &mut dynamic_list,
                        &process,
                    )?;
                    (context.callback.lock().unwrap())(
                        &full_list.iter().map(|(data, _)| *data).collect(),
                    );

                    Some(())
                },
            ),
            true,
        ))
    }

    pub fn hook_list_order_titles(
        &mut self,
        filter: String,
        callback: impl for<'r> FnMut(&Vec<&str>) -> () + Send + Sync + 'a,
    ) -> HookId {
        HookId::ListFilter(self.hook_pool.list_filter.subscrive(
            AsyncCallback::new(
                ListFilterHookContext {
                    database: self.database.clone(),
                    filter,
                    callback: Arc::new(Mutex::new(Box::new(callback))),
                },
                |context, process| {
                    let order_title_filter = Filter::new(&context.filter);
                    let database = context.database.read().unwrap();

                    let mut dynamic_list = Self::list_filter(
                        &order_title_filter,
                        database.iter().flat_map(|(_, exp)| {
                            exp.orders
                                .iter()
                                .map(|order| (order.title.as_str(), order.date))
                        }),
                        &mut vec![],
                        &process,
                    )?;
                    (context.callback.lock().unwrap())(
                        &dynamic_list.iter().map(|(data, _)| *data).collect(),
                    );

                    let full_list = Self::list_filter(
                        &order_title_filter,
                        database.iter_ancient().flat_map(|(_, exp)| {
                            exp.orders
                                .iter()
                                .map(|order| (order.title.as_str(), order.date))
                        }),
                        &mut dynamic_list,
                        &process,
                    )?;
                    (context.callback.lock().unwrap())(
                        &full_list.iter().map(|(data, _)| *data).collect(),
                    );

                    Some(())
                },
            ),
            true,
        ))
    }
}
