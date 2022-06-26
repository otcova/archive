mod expedient;
mod hook;
use self::hook::*;
use crate::chunked_database::*;
pub use crate::collections::UtcDate;
use crate::error::*;
pub use expedient::*;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

pub struct ExpedientDatabase<'a> {
    database: Arc<Mutex<ChunkedDatabase<Expedient>>>,
    hook_pool: HookPool<'a, Event>,
    data_has_changed: bool,
}

#[derive(Debug)]
struct Event {
    pub database: Arc<Mutex<ChunkedDatabase<Expedient>>>,
    pub modifyed_expedient: Uid,
}

const CHUNKED_DATABASE_DYNAMIC_SIZE: usize = 4000;

impl<'a> ExpedientDatabase<'a> {
    pub fn open(path: &PathBuf) -> Result<Self> {
        Ok(Self {
            database: Arc::new(Mutex::new(ChunkedDatabase::open(
                path,
                CHUNKED_DATABASE_DYNAMIC_SIZE,
            )?)),
            hook_pool: HookPool::new(),
            data_has_changed: false,
        })
    }

    pub fn create(path: &PathBuf) -> Result<Self> {
        Ok(Self {
            database: Arc::new(Mutex::new(ChunkedDatabase::create(
                path,
                CHUNKED_DATABASE_DYNAMIC_SIZE,
            )?)),
            hook_pool: HookPool::new(),
            data_has_changed: true,
        })
    }

    pub fn rollback(path: &PathBuf) -> Result<Self> {
        Ok(Self {
            database: Arc::new(Mutex::new(ChunkedDatabase::rollback(
                path,
                CHUNKED_DATABASE_DYNAMIC_SIZE,
            )?)),
            hook_pool: HookPool::new(),
            data_has_changed: false,
        })
    }

    pub fn rollback_info(path: &PathBuf) -> Result<RollbackDateInfo> {
        ChunkedDatabase::<Expedient>::rollback_info(path)
    }

    pub fn read_expedient(&mut self, id: Uid) -> Option<Expedient> {
        self.database.lock().unwrap().read(id).clone()
    }

    pub fn hook_expedient(
        &mut self,
        id: Uid,
        mut callback: impl FnMut(&Option<Expedient>) -> () + Send + Sync + 'a,
    ) -> Option<hook::Id> {
        let db = self.database.lock().unwrap();
        let expedient = db.read(id);
        callback(expedient);
        if expedient.is_some() {
            return Some(self.hook_pool.hook(move |event, release| {
                if event.modifyed_expedient == id {
                    let db = event.database.lock().unwrap();
                    let expedient = db.read(id);
                    callback(expedient);
                    if expedient.is_none() {
                        release();
                    }
                }
            }));
        }
        None
    }

    pub fn hook_expedient_filter(
        &mut self,
        filter: Expedient,
        from: UtcDate,
        limit: usize,
        mut callback: impl FnMut(Vec<(Uid, &Expedient, f32)>) -> () + Send + Sync + 'a,
    ) -> hook::Id {
        let mut dispatch_hook = move |database: &Arc<Mutex<ChunkedDatabase<Expedient>>>| {
            let database = database.lock().unwrap();
            let max_date = from.date_hash();
            let mut expedients: Vec<_> = database
                .iter()
                .filter(|(_, expedient)| expedient.date() <= max_date)
                .map(|(id, expedient)| (id, expedient, expedient.similarity(&filter)))
                .filter(|(_, _, similarity)| *similarity > 0.)
                .collect();
            expedients.sort_unstable_by(|(_, _, a), (_, _, b)| a.partial_cmp(b).unwrap().reverse());
            expedients.truncate(limit);

            callback(expedients);
        };
        dispatch_hook(&self.database.clone());
        self.hook_pool
            .hook(move |event, _| dispatch_hook(&event.database))
    }
    pub fn hook_all_expedients(
        &mut self,
        from: UtcDate,
        limit: usize,
        mut callback: impl FnMut(Vec<(Uid, &Expedient)>) -> () + Send + Sync + 'a,
    ) -> hook::Id {
        let max_date = from.date_hash();

        let mut dispatch_hook = move |database: &Arc<Mutex<ChunkedDatabase<Expedient>>>| {
            let database = database.lock().unwrap();
            let mut sorted_expedients: Vec<_> = database
                .iter()
                .filter(|(_, expedient)| expedient.date() <= max_date)
                .collect();
            sorted_expedients.sort_unstable_by_key(|(_, expedient)| -expedient.date());
            sorted_expedients.truncate(limit);
            callback(sorted_expedients);
        };
        dispatch_hook(&self.database.clone());
        self.hook_pool
            .hook(move |event, _| dispatch_hook(&event.database))
    }
    pub fn hook_all_open_expedients(
        &mut self,
        from: UtcDate,
        limit: usize,
        mut callback: impl FnMut(Vec<(Uid, &Expedient)>) -> () + Send + Sync + 'a,
    ) -> hook::Id {
        let mut dispatch_hook = move |database: &Arc<Mutex<ChunkedDatabase<Expedient>>>| {
            let database = database.lock().unwrap();
            let max_date = from.date_hash();

            let mut sorted_expedients: Vec<_> = database
                .iter()
                .filter(|(_, expedient)| {
                    expedient.global_order_state() != OrderState::Done
                        && expedient.date() <= max_date
                })
                .collect();
            sorted_expedients.sort_unstable_by_key(|(_, expedient)| -expedient.date());
            sorted_expedients.truncate(limit);

            callback(sorted_expedients);
        };
        dispatch_hook(&self.database.clone());
        self.hook_pool
            .hook(move |event, _| dispatch_hook(&event.database))
    }
    pub fn release_hook(&mut self, hook: hook::Id) {
        self.hook_pool.release(hook);
    }
    pub fn release_all_hooks(&mut self) {
        self.hook_pool = HookPool::new();
    }

    pub fn update_expedient(&mut self, id: Uid, expedient: Expedient) {
        self.database.lock().unwrap().update(id, expedient);
        self.dispath_change(id);
    }
    pub fn create_expedient(&mut self, expedient: Expedient) -> Uid {
        let id = self.database.lock().unwrap().push(expedient);
        self.dispath_change(id);
        id
    }
    pub fn delete_expedient(&mut self, id: Uid) {
        self.database.lock().unwrap().delete(id);
        self.dispath_change(id);
    }
    pub fn merge_expedient(&self, _id_a: Uid, _id_b: Uid) {
        todo!()
    }

    fn dispath_change(&mut self, modifyed_expedient: Uid) {
        self.data_has_changed = true;
        self.hook_pool.dispatch(&Event {
            database: self.database.clone(),
            modifyed_expedient,
        });
    }

    /// If data has changes, creates a backup.
    pub fn store(&mut self) -> Result<()> {
        if self.data_has_changed {
            self.data_has_changed = false;
            self.database.lock().unwrap().store()?;
        }
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
            db.hook_expedient(id, |exp| db_expedient = exp.clone());
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
                    1 => assert_eq!(&Some(expedient_a.clone()), exp),
                    2 => assert_eq!(&Some(expedient_b.clone()), exp),
                    3 => assert_eq!(&None, exp),
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
                    1 => assert_eq!(&Some(expedient_a.clone()), exp),
                    _ => panic!("To many calls"),
                }
            });
            db.hook_expedient(id, |exp| {
                call_count_b += 1;
                match call_count_b {
                    1 => assert_eq!(&Some(expedient_a.clone()), exp),
                    _ => panic!("To many calls"),
                }
            });
            db.release_all_hooks();
            db.update_expedient(id, expedient_b.clone());
            db.delete_expedient(id);
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
                    1 => assert_eq!(&Some(expedient_a.clone()), exp),
                    _ => panic!("To many calls"),
                }
            });
            db.release_hook(hook_id.unwrap());
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
        let id_5 = Uid::DYNAMIC(5);
        
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
                    3 => assert_eq!(vec![id_4, id_5, id_2], expedients_ids),
                    4 => assert_eq!(vec![id_4, id_5, id_2], expedients_ids),
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
                    3 => assert_eq!(vec![id_0, id_1, id_3, id_4, id_5, id_2], expedients_ids),
                    4 => assert_eq!(vec![id_0, id_1, id_3, id_4, id_5, id_2], expedients_ids),
                    5 => assert_eq!(vec![id_0, id_1, id_3, id_5, id_2], expedients_ids),
                    _ => panic!(),
                }
            },
        );

        db.update_expedient(id_1, expedient_done);
        db.create_expedient(expedient_todo.clone());
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
        db.create_expedient(expedient_0);
        let id_1 = db.create_expedient(expedient_1.clone());
        db.create_expedient(expedient_2);
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
                assert_eq!(0.5, filter[0].2);

                assert_eq!(id_1, filter[1].0);
                assert_eq!(expedient_1, *filter[1].1);
                assert_eq!(0.375, filter[1].2);
            },
        );
        drop(db);

        assert!(hook_has_triggered);
    }

    #[test]
    fn store_do_not_create_files_when_data_is_not_changed() {
        let tempdir = TempDir::new();
        {
            let mut database = ExpedientDatabase::create(&tempdir.path).unwrap();
            database.store().unwrap();
            database.store().unwrap();
            database.store().unwrap();
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
            database.store().unwrap();
            sleep_for(1100);
            database.store().unwrap();
            database.store().unwrap();
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
