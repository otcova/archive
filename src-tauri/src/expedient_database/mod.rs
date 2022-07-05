mod expedient;
mod observable;
use crate::chunked_database::*;
pub use crate::collections::UtcDate;
use crate::error::*;
pub use expedient::*;
use observable::*;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::{Arc, Mutex, RwLock};

pub struct ExpedientDatabase<'a> {
    database: Arc<RwLock<ChunkedDatabase<Expedient>>>,
    expedients_observable: Observable<ExpedientHookContext<'a>>,
    expedients_list_observable: Observable<ExpedientListHookContext<'a>>,
    expedients_filter_list_observable: Observable<ExpedientFilterListHookContext<'a>>,
}

#[derive(Clone)]
struct ExpedientHookContext<'a> {
    pub database: Arc<RwLock<ChunkedDatabase<Expedient>>>,
    pub expedient_id: Uid,
    pub callback: Arc<Mutex<Box<dyn for<'r> FnMut(Option<&'r Expedient>) + Send + Sync + 'a>>>,
}

#[derive(Clone)]
struct ExpedientListHookContext<'a> {
    pub database: Arc<RwLock<ChunkedDatabase<Expedient>>>,
    pub max_date: i32,
    pub limit_len: usize,
    pub callback: Arc<Mutex<Box<dyn for<'r> FnMut(Vec<(Uid, &'r Expedient)>) + Send + Sync + 'a>>>,
}

#[derive(Clone)]
struct ExpedientFilterListHookContext<'a> {
    pub database: Arc<RwLock<ChunkedDatabase<Expedient>>>,
    pub max_date: i32,
    pub limit_len: usize,
    pub filter: Expedient,
    pub callback:
        Arc<Mutex<Box<dyn for<'r> FnMut(Vec<(Uid, &'r Expedient, f32)>) + Send + Sync + 'a>>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HookId {
    Expedient(Id),
    ExpedientList(Id),
    ExpedientFilterList(Id),
}

const CHUNKED_DATABASE_DYNAMIC_SIZE: usize = 6000;

impl<'a> ExpedientDatabase<'a> {
    pub fn open(path: &PathBuf) -> Result<Self> {
        Ok(Self {
            database: Arc::new(RwLock::new(ChunkedDatabase::open(
                path,
                CHUNKED_DATABASE_DYNAMIC_SIZE,
            )?)),
            expedients_observable: Observable::new(),
            expedients_list_observable: Observable::new(),
            expedients_filter_list_observable: Observable::new(),
        })
    }

    pub fn create(path: &PathBuf) -> Result<Self> {
        Ok(Self {
            database: Arc::new(RwLock::new(ChunkedDatabase::create(
                path,
                CHUNKED_DATABASE_DYNAMIC_SIZE,
            )?)),
            expedients_observable: Observable::new(),
            expedients_list_observable: Observable::new(),
            expedients_filter_list_observable: Observable::new(),
        })
    }

    pub fn rollback(path: &PathBuf) -> Result<Self> {
        Ok(Self {
            database: Arc::new(RwLock::new(ChunkedDatabase::rollback(
                path,
                CHUNKED_DATABASE_DYNAMIC_SIZE,
            )?)),
            expedients_observable: Observable::new(),
            expedients_list_observable: Observable::new(),
            expedients_filter_list_observable: Observable::new(),
        })
    }

    pub fn rollback_info(path: &PathBuf) -> Result<RollbackDateInfo> {
        ChunkedDatabase::<Expedient>::rollback_info(path)
    }

    pub fn read_expedient(&self, id: Uid) -> Option<Expedient> {
        self.database
            .read()
            .unwrap()
            .read(id)
            .map(|exp| exp.clone())
    }

    pub fn hook_expedient(
        &mut self,
        id: Uid,
        callback: impl for<'r> FnMut(Option<&'r Expedient>) -> () + Send + Sync + 'a,
    ) -> HookId {
        HookId::Expedient(self.expedients_observable.subscrive(
            Callback {
                callback: |context| {
                    let database = context.database.read().unwrap();
                    let expedient = database.read(context.expedient_id);
                    (context.callback.lock().unwrap())(expedient)
                },
                context: ExpedientHookContext {
                    database: self.database.clone(),
                    expedient_id: id,
                    callback: Arc::new(Mutex::new(Box::new(callback))),
                },
            },
            true,
        ))
    }

    pub fn hook_all_expedients(
        &mut self,
        from_date: UtcDate,
        limit_len: usize,
        callback: impl for<'r> FnMut(Vec<(Uid, &'r Expedient)>) -> () + Send + Sync + 'a,
    ) -> HookId {
        HookId::ExpedientList(self.expedients_list_observable.subscrive(
            Callback {
                callback: |context| {
                    let database = context.database.read().unwrap();

                    let mut expedient_list: Vec<_> = database
                        .iter()
                        .filter(|(_, expedient)| expedient.date() <= context.max_date)
                        .collect();

                    expedient_list.sort_unstable_by_key(|(_, expedient)| -expedient.date());
                    expedient_list.truncate(context.limit_len);
                    (context.callback.lock().unwrap())(expedient_list)

                    // TODO: check on ancient database
                },
                context: ExpedientListHookContext {
                    database: self.database.clone(),
                    max_date: from_date.date_hash(),
                    limit_len,
                    callback: Arc::new(Mutex::new(Box::new(callback))),
                },
            },
            true,
        ))
    }
    pub fn hook_all_open_expedients(
        &mut self,
        from_date: UtcDate,
        limit_len: usize,
        callback: impl for<'r> FnMut(Vec<(Uid, &'r Expedient)>) -> () + Send + Sync + 'a,
    ) -> HookId {
        HookId::ExpedientList(self.expedients_list_observable.subscrive(
            Callback {
                callback: |context| {
                    let database = context.database.read().unwrap();

                    let mut expedient_list: Vec<_> = database
                        .iter()
                        .filter(|(_, expedient)| {
                            expedient.date() <= context.max_date
                                && expedient.global_order_state() != OrderState::Done
                        })
                        .collect();

                    expedient_list.sort_unstable_by_key(|(_, expedient)| {
                        if expedient.global_order_state() == OrderState::Todo {
                            i32::MAX / 2 - expedient.date()
                        } else {
                            -expedient.date()
                        }
                    });
                    expedient_list.truncate(context.limit_len);
                    (context.callback.lock().unwrap())(expedient_list)

                    // TODO: check on ancient database
                },
                context: ExpedientListHookContext {
                    database: self.database.clone(),
                    max_date: from_date.date_hash(),
                    limit_len,
                    callback: Arc::new(Mutex::new(Box::new(callback))),
                },
            },
            true,
        ))
    }
    pub fn hook_expedient_filter(
        &mut self,
        filter: Expedient,
        from_date: UtcDate,
        limit_len: usize,
        callback: impl for<'r> FnMut(Vec<(Uid, &'r Expedient, f32)>) -> () + Send + Sync + 'a,
    ) -> HookId {
        HookId::ExpedientFilterList(self.expedients_filter_list_observable.subscrive(
            Callback {
                callback: |context| {
                    let database = context.database.read().unwrap();

                    let mut expedient_list: Vec<_> = database
                        .iter()
                        .map(|(id, expedient)| {
                            (id, expedient, expedient.similarity(&context.filter))
                        })
                        .filter(|(_, expedient, similarity)| {
                            expedient.date() <= context.max_date && *similarity > 0.
                        })
                        .collect();
                    expedient_list.sort_unstable_by_key(|(_, expedient, similarity)| {
                        (-(1 << 24) as f32 * similarity) as i32 - expedient.date()
                    });
                    expedient_list.truncate(context.limit_len);
                    (context.callback.lock().unwrap())(expedient_list)

                    // TODO: check on ancient database
                },
                context: ExpedientFilterListHookContext {
                    database: self.database.clone(),
                    max_date: from_date.date_hash(),
                    limit_len,
                    filter,
                    callback: Arc::new(Mutex::new(Box::new(callback))),
                },
            },
            true,
        ))
    }

    pub fn release_hook(&mut self, hook_id: HookId) {
        match hook_id {
            HookId::Expedient(id) => self.expedients_observable.unsubscrive(id),
            HookId::ExpedientList(id) => self.expedients_list_observable.unsubscrive(id),
            HookId::ExpedientFilterList(id) => {
                self.expedients_filter_list_observable.unsubscrive(id)
            }
        }
    }
    pub fn release_all_hooks(&mut self) {
        self.expedients_observable = Observable::new();
        self.expedients_list_observable = Observable::new();
    }

    pub fn update_expedient(&mut self, id: Uid, expedient: Expedient) {
        self.database.write().unwrap().update(id, expedient);
        self.dispath_change();
    }
    pub fn create_expedient(&mut self, expedient: Expedient) -> Uid {
        let id = self.database.write().unwrap().push(expedient);
        self.dispath_change();
        id
    }
    pub fn delete_expedient(&mut self, id: Uid) {
        self.database.write().unwrap().delete(id);
        self.dispath_change();
    }
    pub fn merge_expedient(&self, _id_a: Uid, _id_b: Uid) {
        todo!()
    }

    fn dispath_change(&mut self) {
        self.expedients_observable.trigger();
        // block_on(join!(
        self.expedients_filter_list_observable.trigger();
        self.expedients_list_observable.trigger();
        // ));
    }

    /// If data has changes, creates a backup.
    pub fn save(&mut self) -> Result<()> {
        self.database.write().unwrap().save()?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_utils::*;

    #[test]
    fn create_close_and_open_database() {
        let tempdir = TempDir::new();
        {
            ExpedientDatabase::create(&tempdir.path).unwrap();
        }
        ExpedientDatabase::open(&tempdir.path).unwrap();
    }

    #[test]
    fn create_and_hook_expedient() {
        let tempdir = TempDir::new();

        let expedient = Expedient {
            description: String::from("Vermell Audi"),
            license_plate: String::from(""),
            model: String::from(""),
            orders: vec![],
            users: vec![],
            vin: String::from(""),
            date: UtcDate {
                year: 2010,
                month: 1,
                day: 3,
                hour: 23,
            },
        };
        let mut db_expedient = None;

        {
            let mut db = ExpedientDatabase::create(&tempdir.path).unwrap();
            let id = db.create_expedient(expedient.clone());
            db.hook_expedient(id, |exp| db_expedient = exp.map(|d| d.clone()));
        }

        assert_eq!(expedient, db_expedient.unwrap());
    }

    #[test]
    fn create_hook_update_and_delete_expedient() {
        let tempdir = TempDir::new();

        let expedient_a = Expedient {
            description: String::from("Eduardo Dato"),
            license_plate: String::from(""),
            model: String::from(""),
            orders: vec![],
            users: vec![],
            vin: String::from(""),
            date: UtcDate {
                year: 1921,
                month: 3,
                day: 8,
                hour: 21,
            },
        };

        let expedient_b = Expedient {
            description: String::from("Eduardo Pedro"),
            license_plate: String::from(""),
            model: String::from(""),
            orders: vec![],
            users: vec![],
            vin: String::from(""),
            date: UtcDate {
                year: 1921,
                month: 3,
                day: 8,
                hour: 21,
            },
        };

        let mut call_count = 0;

        {
            let mut db = ExpedientDatabase::create(&tempdir.path).unwrap();
            let id = db.create_expedient(expedient_a.clone());
            db.hook_expedient(id, |exp| {
                call_count += 1;
                match call_count {
                    1 => assert_eq!(Some(&expedient_a.clone()), exp),
                    2 => assert_eq!(Some(&expedient_b.clone()), exp),
                    3 => assert_eq!(None, exp),
                    _ => panic!("To many calls"),
                }
            });
            db.update_expedient(id, expedient_b.clone());
            db.delete_expedient(id);
        }
        assert_eq!(3, call_count, "Expected only 3 calls");
    }

    #[test]
    fn release_all_hooks() {
        let tempdir = TempDir::new();

        let expedient_a = Expedient {
            description: String::from("Eduardo Dato"),
            license_plate: String::from(""),
            model: String::from(""),
            orders: vec![],
            users: vec![],
            vin: String::from(""),
            date: UtcDate {
                year: 1921,
                month: 3,
                day: 8,
                hour: 21,
            },
        };

        let expedient_b = Expedient {
            description: String::from("Eduardo Pedro"),
            license_plate: String::from(""),
            model: String::from(""),
            orders: vec![],
            users: vec![],
            vin: String::from(""),
            date: UtcDate {
                year: 1921,
                month: 3,
                day: 8,
                hour: 21,
            },
        };

        let mut call_count_a = 0;
        let mut call_count_b = 0;

        {
            let mut db = ExpedientDatabase::create(&tempdir.path).unwrap();
            let id = db.create_expedient(expedient_a.clone());
            db.hook_expedient(id, |exp| {
                call_count_a += 1;
                match call_count_a {
                    1 => assert_eq!(Some(&expedient_a.clone()), exp),
                    _ => panic!("To many calls"),
                }
            });
            db.hook_expedient(id, |exp| {
                call_count_b += 1;
                match call_count_b {
                    1 => assert_eq!(Some(&expedient_a.clone()), exp),
                    _ => panic!("To many calls"),
                }
            });
            db.release_all_hooks();
            (db.update_expedient(id, expedient_b.clone()));
            (db.delete_expedient(id));
        }
        assert_eq!(1, call_count_a, "Expected only one call per hook");
        assert_eq!(1, call_count_b, "Expected only one call per hook");
    }

    #[test]
    fn release_hook() {
        let tempdir = TempDir::new();

        let expedient_a = Expedient {
            description: String::from("Eduardo Dato"),
            license_plate: String::from(""),
            model: String::from(""),
            orders: vec![],
            users: vec![],
            vin: String::from(""),
            date: UtcDate {
                year: 1921,
                month: 3,
                day: 8,
                hour: 21,
            },
        };

        let expedient_b = Expedient {
            description: String::from("Eduardo Pedro"),
            license_plate: String::from(""),
            model: String::from(""),
            orders: vec![],
            users: vec![],
            vin: String::from(""),
            date: UtcDate {
                year: 1921,
                month: 3,
                day: 8,
                hour: 21,
            },
        };

        let mut call_count = 0;

        {
            let mut db = ExpedientDatabase::create(&tempdir.path).unwrap();
            let id = db.create_expedient(expedient_a.clone());
            let hook_id = db.hook_expedient(id, |exp| {
                call_count += 1;
                match call_count {
                    1 => assert_eq!(Some(&expedient_a.clone()), exp),
                    _ => panic!("To many calls"),
                }
            });
            db.release_hook(hook_id);
            db.update_expedient(id, expedient_b.clone());
            db.delete_expedient(id);
        }
        assert_eq!(1, call_count, "Expected only 1 call");
    }

    #[test]
    fn all_and_all_open_expedients_hook() {
        let tempdir = TempDir::new();

        let expedient_done = Expedient {
            description: String::from("Eduardo Dato"),
            license_plate: String::from(""),
            model: String::from(""),
            orders: vec![Order {
                date: UtcDate {
                    year: 2921,
                    month: 3,
                    day: 8,
                    hour: 22,
                },
                title: String::from(""),
                description: String::from(""),
                state: OrderState::Done,
            }],
            users: vec![],
            vin: String::from(""),
            date: UtcDate {
                year: 1921,
                month: 3,
                day: 8,
                hour: 21,
            },
        };

        let expedient_todo = Expedient {
            description: String::from("Eduardo Pedro"),
            license_plate: String::from(""),
            model: String::from(""),
            orders: vec![Order {
                date: UtcDate {
                    year: 1921,
                    month: 3,
                    day: 8,
                    hour: 22,
                },
                title: String::from(""),
                description: String::from(""),
                state: OrderState::Todo,
            }],
            users: vec![],
            vin: String::from(""),
            date: UtcDate {
                year: 1921,
                month: 3,
                day: 8,
                hour: 21,
            },
        };

        let old_expedient_todo = Expedient {
            description: String::from("Eduardo Pedro"),
            license_plate: String::from(""),
            model: String::from(""),
            orders: vec![Order {
                date: UtcDate {
                    year: 1821,
                    month: 3,
                    day: 8,
                    hour: 22,
                },
                title: String::from(""),
                description: String::from(""),
                state: OrderState::Todo,
            }],
            users: vec![],
            vin: String::from(""),
            date: UtcDate {
                year: 1721,
                month: 3,
                day: 8,
                hour: 21,
            },
        };

        let expedient_urgent = Expedient {
            description: String::from("Eduardo Pedro"),
            license_plate: String::from(""),
            model: String::from(""),
            orders: vec![Order {
                date: UtcDate {
                    year: 1721,
                    month: 3,
                    day: 8,
                    hour: 22,
                },
                title: String::from(""),
                description: String::from(""),
                state: OrderState::Urgent,
            }],
            users: vec![],
            vin: String::from(""),
            date: UtcDate {
                year: 1421,
                month: 3,
                day: 8,
                hour: 21,
            },
        };

        let future_expedient = Expedient {
            description: String::from("Eduardo Pedro"),
            license_plate: String::from(""),
            model: String::from(""),
            orders: vec![Order {
                date: UtcDate {
                    year: 7821,
                    month: 3,
                    day: 8,
                    hour: 22,
                },
                title: String::from(""),
                description: String::from(""),
                state: OrderState::Todo,
            }],
            users: vec![],
            vin: String::from(""),
            date: UtcDate {
                year: 1721,
                month: 3,
                day: 8,
                hour: 21,
            },
        };

        let mut db = ExpedientDatabase::create(&tempdir.path).unwrap();
        let id_0 = db.create_expedient(expedient_done.clone());
        let id_1 = db.create_expedient(expedient_todo.clone());
        let id_2 = db.create_expedient(old_expedient_todo.clone());
        let id_3 = db.create_expedient(expedient_done.clone());
        let id_4 = db.create_expedient(expedient_todo.clone());
        let id_5 = Uid::DYNAMIC(Id {
            index: 5,
            identifier: 6,
        });

        let mut hook_all_open_has_been_triggered = 0;
        let mut hook_all_has_been_triggered = 0;

        db.hook_all_open_expedients(
            UtcDate {
                year: 4021,
                month: 3,
                day: 8,
                hour: 21,
            },
            100,
            |expedeints| {
                hook_all_open_has_been_triggered += 1;
                let expedients_ids = expedeints.into_iter().map(|(id, _)| id).collect::<Vec<_>>();

                match hook_all_open_has_been_triggered {
                    1 => assert_eq!(vec![id_1, id_4, id_2], expedients_ids),
                    2 => assert_eq!(vec![id_4, id_2], expedients_ids),
                    3 => assert_eq!(vec![id_5, id_4, id_2], expedients_ids),
                    4 => assert_eq!(vec![id_5, id_4, id_2], expedients_ids),
                    5 => assert_eq!(vec![id_5, id_2], expedients_ids),
                    _ => panic!(),
                }
            },
        );

        db.hook_all_expedients(
            UtcDate {
                year: 4021,
                month: 3,
                day: 8,
                hour: 21,
            },
            100,
            |expedeints| {
                hook_all_has_been_triggered += 1;

                let expedients_ids = expedeints.into_iter().map(|(id, _)| id).collect::<Vec<_>>();

                match hook_all_has_been_triggered {
                    1 => assert_eq!(vec![id_0, id_3, id_1, id_4, id_2], expedients_ids),
                    2 => assert_eq!(vec![id_0, id_1, id_3, id_4, id_2], expedients_ids),
                    3 => assert_eq!(vec![id_0, id_1, id_3, id_4, id_2, id_5], expedients_ids),
                    4 => assert_eq!(vec![id_0, id_1, id_3, id_4, id_2, id_5], expedients_ids),
                    5 => assert_eq!(vec![id_0, id_1, id_3, id_2, id_5], expedients_ids),
                    _ => panic!(),
                }
            },
        );

        db.update_expedient(id_1, expedient_done);
        db.create_expedient(expedient_urgent.clone());
        db.create_expedient(future_expedient.clone());
        db.delete_expedient(id_4);

        drop(db);

        assert_eq!(5, hook_all_open_has_been_triggered);
        assert_eq!(5, hook_all_has_been_triggered);
    }

    #[test]
    fn filter_expedients_hook() {
        let tempdir = TempDir::new();

        let expedient_0 = Expedient {
            description: String::from("Jeronimo Dato"),
            license_plate: String::from(""),
            model: String::from(""),
            orders: vec![],
            users: vec![],
            vin: String::from(""),
            date: UtcDate {
                year: 1921,
                month: 3,
                day: 8,
                hour: 21,
            },
        };

        let expedient_1 = Expedient {
            description: String::from("Eduardo Pedro"),
            license_plate: String::from(""),
            model: String::from(""),
            orders: vec![],
            users: vec![],
            vin: String::from(""),
            date: UtcDate {
                year: 1921,
                month: 3,
                day: 8,
                hour: 21,
            },
        };

        let expedient_2 = Expedient {
            description: String::from("Eduardo Pedro"),
            license_plate: String::from(""),
            model: String::from("Car"),
            orders: vec![],
            users: vec![],
            vin: String::from(""),
            date: UtcDate {
                year: 3921,
                month: 3,
                day: 8,
                hour: 21,
            },
        };

        let expedient_3 = Expedient {
            description: String::from("Eduardo Dato"),
            license_plate: String::from(""),
            model: String::from("Car"),
            orders: vec![],
            users: vec![],
            vin: String::from(""),
            date: UtcDate {
                year: 1921,
                month: 3,
                day: 8,
                hour: 21,
            },
        };

        let expedient_filter = Expedient {
            description: String::from("Pedro"),
            license_plate: String::from(""),
            model: String::from("Car"),
            orders: vec![],
            users: vec![],
            vin: String::from(""),
            date: UtcDate {
                year: 2921,
                month: 4,
                day: 2,
                hour: 11,
            },
        };

        let mut hook_has_triggered = false;

        let mut db = ExpedientDatabase::create(&tempdir.path).unwrap();
        (db.create_expedient(expedient_0));
        let id_1 = db.create_expedient(expedient_1.clone());
        (db.create_expedient(expedient_2));
        let id_3 = db.create_expedient(expedient_3.clone());

        db.hook_expedient_filter(
            expedient_filter,
            UtcDate {
                year: 2900,
                month: 3,
                day: 2,
                hour: 0,
            },
            10,
            |filter| {
                hook_has_triggered = true;
                assert_eq!(2, filter.len());

                assert_eq!(id_3, filter[0].0);
                assert_eq!(expedient_3, *filter[0].1);

                assert_eq!(id_1, filter[1].0);
                assert_eq!(expedient_1, *filter[1].1);
            },
        );
        drop(db);

        assert!(hook_has_triggered);
    }

    #[test]
    fn save_do_not_create_files_when_data_is_not_changed() {
        let tempdir = TempDir::new();
        {
            let mut database = ExpedientDatabase::create(&tempdir.path).unwrap();
            database.save().unwrap();
            database.create_expedient(Expedient {
                description: String::from("Pedro"),
                license_plate: String::from(""),
                model: String::from("Car"),
                orders: vec![],
                users: vec![],
                vin: String::from(""),
                date: UtcDate {
                    year: 2921,
                    month: 4,
                    day: 2,
                    hour: 11,
                },
            });
            database.save().unwrap();
            sleep_for(1100);
            database.save().unwrap();
            database.save().unwrap();
        }

        let year = crate::database::Instant::now().year().to_string();

        // Check for folders 'ancient' and 'database'
        assert_eq!(
            2,
            std::fs::read_dir(&tempdir.path)
                .unwrap()
                .into_iter()
                .count()
        );

        // Check that folder 'ancient' only contains 2 item (lock file and folder {year})
        assert_eq!(
            2,
            std::fs::read_dir(tempdir.path.join("ancient"))
                .unwrap()
                .into_iter()
                .count()
        );
        // Check that folder 'ancient/{year}' containt only 2 files
        assert_eq!(
            2,
            std::fs::read_dir(tempdir.path.join("ancient").join(&year))
                .unwrap()
                .into_iter()
                .count()
        );

        // Check that folder 'dynamic' only contains 2 item (lock file and folder {year})
        assert_eq!(
            2,
            std::fs::read_dir(tempdir.path.join("dynamic"))
                .unwrap()
                .into_iter()
                .count()
        );
        // Check that folder 'dynamic/{year}' containt only 2 files
        assert_eq!(
            2,
            std::fs::read_dir(tempdir.path.join("dynamic").join(&year))
                .unwrap()
                .into_iter()
                .count()
        );
    }
}
