mod expedient;
mod filter;
mod hooks;
use crate::chunked_database::*;
pub use crate::collections::UtcDate;
use crate::error::*;
pub use expedient::*;
pub use hooks::*;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

pub struct ExpedientDatabase<'a> {
    database: Arc<RwLock<ChunkedDatabase<Expedient>>>,
    hook_pool: HookPool<'a>,
}

const CHUNKED_DATABASE_DYNAMIC_SIZE: usize = 6000;

impl<'a> ExpedientDatabase<'a> {
    pub fn open(path: &PathBuf) -> Result<Self> {
        Ok(Self {
            database: Arc::new(RwLock::new(ChunkedDatabase::open(
                path,
                CHUNKED_DATABASE_DYNAMIC_SIZE,
            )?)),
            hook_pool: Default::default(),
        })
    }

    pub fn create(path: &PathBuf) -> Result<Self> {
        Ok(Self {
            database: Arc::new(RwLock::new(ChunkedDatabase::create(
                path,
                CHUNKED_DATABASE_DYNAMIC_SIZE,
            )?)),
            hook_pool: Default::default(),
        })
    }

    pub fn rollback(path: &PathBuf) -> Result<Self> {
        Ok(Self {
            database: Arc::new(RwLock::new(ChunkedDatabase::rollback(
                path,
                CHUNKED_DATABASE_DYNAMIC_SIZE,
            )?)),
            hook_pool: Default::default(),
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

    pub fn update_expedient(&mut self, id: Uid, expedient: Expedient) {
        self.interrupt_dispatch();
        self.database.write().unwrap().update(id, expedient);
        self.dispatch_change();
    }
    pub fn create_expedient(&mut self, expedient: Expedient) -> Uid {
        self.interrupt_dispatch();
        let id = self.database.write().unwrap().push(expedient);
        self.dispatch_change();
        id
    }
    pub fn delete_expedient(&mut self, id: Uid) {
        self.interrupt_dispatch();
        self.database.write().unwrap().delete(id);
        self.dispatch_change();
    }
    pub fn merge_expedient(&self, _id_a: Uid, _id_b: Uid) {
        todo!()
    }

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
            description: "Vermell Audi".into(),
            license_plate: "".into(),
            model: "".into(),
            orders: vec![],
            user: "".into(),
            vin: "".into(),
            date: UtcDate::ymdh(2010, 1, 3, 23),
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
            description: "Eduardo Dato".into(),
            license_plate: "".into(),
            model: "".into(),
            orders: vec![],
            user: "".into(),
            vin: "".into(),
            date: UtcDate::ymdh(1921, 3, 8, 21),
        };

        let expedient_b = Expedient {
            description: "Eduardo Pedro".into(),
            license_plate: "".into(),
            model: "".into(),
            orders: vec![],
            user: "".into(),
            vin: "".into(),
            date: UtcDate::ymdh(1921, 3, 8, 21),
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
                };
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
            description: "Eduardo Dato".into(),
            license_plate: "".into(),
            model: "".into(),
            orders: vec![],
            user: "".into(),
            vin: "".into(),
            date: UtcDate::ymdh(1921, 3, 8, 21),
        };

        let expedient_b = Expedient {
            description: "Eduardo Pedro".into(),
            license_plate: "".into(),
            model: "".into(),
            orders: vec![],
            user: "".into(),
            vin: "".into(),
            date: UtcDate::ymdh(1921, 3, 8, 21),
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
            description: "Eduardo Dato".into(),
            license_plate: "".into(),
            model: "".into(),
            orders: vec![],
            user: "".into(),
            vin: "".into(),
            date: UtcDate::ymdh(1921, 3, 8, 21),
        };

        let expedient_b = Expedient {
            description: "Eduardo Pedro".into(),
            license_plate: "".into(),
            model: "".into(),
            orders: vec![],
            user: "".into(),
            vin: "".into(),
            date: UtcDate::ymdh(1921, 3, 8, 21),
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
    fn filter_expedients_hook() {
        let tempdir = TempDir::new();

        let expedient_0 = Expedient {
            description: "Jeronimo Dato".into(),
            license_plate: "".into(),
            model: "".into(),
            orders: vec![],
            user: "".into(),
            vin: "".into(),
            date: UtcDate::ymdh(1921, 3, 8, 21),
        };

        let expedient_1 = Expedient {
            description: "Eduardo Pedro".into(),
            license_plate: "".into(),
            model: "".into(),
            orders: vec![],
            user: "".into(),
            vin: "".into(),
            date: UtcDate::ymdh(1921, 3, 8, 21),
        };

        let expedient_2 = Expedient {
            description: "Eduardo Pedro".into(),
            license_plate: "".into(),
            model: "Car".into(),
            orders: vec![],
            user: "".into(),
            vin: "".into(),
            date: UtcDate::ymdh(3921, 3, 8, 21),
        };

        let expedient_3 = Expedient {
            description: "Eduardo Dato".into(),
            license_plate: "".into(),
            model: "Car".into(),
            orders: vec![],
            user: "".into(),
            vin: "".into(),
            date: UtcDate::ymdh(1921, 3, 8, 21),
        };

        let expedient_filter = Expedient {
            description: "Pedro".into(),
            license_plate: "".into(),
            model: "Car".into(),
            orders: vec![],
            user: "".into(),
            vin: "".into(),
            date: UtcDate::ymdh(2921, 4, 2, 11),
        };

        let mut hook_has_triggered = false;

        let mut db = ExpedientDatabase::create(&tempdir.path).unwrap();
        db.create_expedient(expedient_0);
        let id_1 = db.create_expedient(expedient_1.clone());
        let id_2 = db.create_expedient(expedient_2.clone());
        let id_3 = db.create_expedient(expedient_3.clone());

        db.hook_list_expedients(
            ListExpedientsHookOptions {
                filter: expedient_filter,
                max_list_len: 10,
            },
            |filter| {
                hook_has_triggered = true;
                assert_eq!(3, filter.len());

                assert_eq!(id_2, filter[0].0);
                assert_eq!(expedient_2, *filter[0].1);

                assert_eq!(id_3, filter[1].0);
                assert_eq!(expedient_3, *filter[1].1);

                assert_eq!(id_1, filter[2].0);
                assert_eq!(expedient_1, *filter[2].1);
            },
        );
        sleep_for(20);
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
                description: "Pedro".into(),
                license_plate: "".into(),
                model: "Car".into(),
                orders: vec![],
                user: "".into(),
                vin: "".into(),
                date: UtcDate::ymdh(2921, 4, 3, 11),
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
