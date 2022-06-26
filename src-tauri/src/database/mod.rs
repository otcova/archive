#![allow(dead_code)]

mod file;
mod time;

pub use crate::error::{ErrorKind, Result};
use serde::{de::DeserializeOwned, Serialize};
use std::{
    fs::{create_dir_all, remove_file},
    path::PathBuf,
};
pub use time::*;

#[derive(Debug)]
pub struct Database<T: Default + DeserializeOwned + Serialize> {
    lock: file::Lock,
    pub data: T,
    path: PathBuf,
}

#[derive(Debug, Serialize)]
pub struct RollbackDateInfo {
    pub newest_instant: Instant,
    pub rollback_instant: Instant,
}

impl<T> Database<T>
where
    T: Default + DeserializeOwned + Serialize,
{
    ///# Errors
    /// - `Collision`: When lock fails.
    ///
    ///- `NotFound`: When database doesn't exist.
    /// If you want to create a database use `create`.
    ///
    /// - `DataIsCorrupted`: When the last instance of database is corrupted.
    /// For rollback use `rollback`.
    pub fn open(path: &PathBuf) -> Result<Self> {
        let lock = file::Lock::directory(path)?;
        let data: T = file::load_newest(path)?;
        Ok(Self {
            lock,
            data,
            path: path.clone(),
        })
    }

    ///# Errors
    /// - `Collision`: When lock fails.
    /// - `AlreadyExist`: When already exists a database.
    pub fn create(path: &PathBuf) -> Result<Self> {
        if path.exists() {
            if !path.read_dir().unwrap().next().is_none() {
                return ErrorKind::AlreadyExist.into();
            }
        } else {
            create_dir_all(&path)?;
        }

        let lock = file::Lock::directory(&path)?;

        let data: T = Default::default();
        Ok(Self {
            lock,
            data,
            path: path.clone(),
        })
    }

    /// Usualy, after a `rollback`, we might whant to see some
    /// information about the rollback date.
    ///
    ///# Errors
    /// - `Collision`: When lock fails.
    /// - `NotFound`: When database doesn't exist or there are no backups.
    pub fn rollback_info(path: &PathBuf) -> Result<RollbackDateInfo> {
        let _lock = file::Lock::directory(path)?;

        let newest_instant = file::select_backup(path, |(_, i)| Some(i))?
            .map_or_else::<Result<Instant>, _, _>(|| ErrorKind::NotFound.into(), |i| Ok(i))?;
        let rollback_instant = file::instant_of_newest_noncurrupted::<T>(path)?;

        Ok(RollbackDateInfo {
            newest_instant,
            rollback_instant,
        })
    }

    /// Open database from the last noncorrupted backup.
    ///
    /// # Errors
    /// - `Collision`: When lock fails.
    /// - `NotFound`: When database doesn't exist or there's no backups.
    pub fn rollback(path: &PathBuf) -> Result<Self> {
        let lock = file::Lock::directory(path)?;
        let data: T = file::load_newest_noncurrupted(path)?;
        Ok(Self {
            lock,
            data,
            path: path.clone(),
        })
    }

    pub fn store(&self) -> Result<PathBuf> {
        self.delete_old_backups()?;
        file::save_data(&self.path, &self.data)
    }

    /// When `store`, a new file is created and the old ones are keeped as backups.
    /// To not run out of memory, some old backups are deleted.
    ///
    /// # What is conserved
    /// - Backups of the newest day are ignored (conserved).
    /// - Every month (exept of newest) will have only the newest backup of that month.
    fn delete_old_backups(&self) -> Result<()> {
        if let Some(newest_backup_instant) = file::select_backup(&self.path, |(_, i)| Some(i))? {
            let newest_backup_day = newest_backup_instant.truncate_time();
            let mut past_year_month = (0, 0);
            let mut files_to_delete = vec![];
            files_to_delete.reserve(20);

            file::select_backup::<(), _>(&self.path, |(file, instant)| {
                if instant < newest_backup_day {
                    if past_year_month.0 == instant.year() && past_year_month.1 == instant.month() {
                        files_to_delete.push(file);
                    } else {
                        past_year_month = (instant.year(), instant.month());
                    }
                }
                None
            })?;

            for file_path in files_to_delete {
                remove_file(file_path)?;
            }
        }

        Ok(())
    }
}

impl<T> Drop for Database<T>
where
    T: Default + DeserializeOwned + Serialize,
{
    fn drop(&mut self) {
        self.store().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;
    use std::fs::create_dir_all;

    #[test]
    fn open_database_two_times_to_create_a_collision() {
        let tempdir = TempDir::new();
        let _database = Database::<i32>::create(&tempdir.path).unwrap();
        let error = Database::<i32>::open(&tempdir.path);
        assert_eq!("Err(Collision)", format!("{:?}", error));
    }

    #[test]
    fn open_database_on_empty_dir() {
        let tempdir = TempDir::new();
        let error = Database::<i32>::open(&tempdir.path);
        assert_eq!("Err(NotFound)", format!("{:?}", error));
    }

    #[test]
    fn create_database_on_empty_dir() {
        let tempdir = TempDir::new();
        let database = Database::<i32>::create(&tempdir.path);
        assert!(database.is_ok());
    }

    #[test]
    fn create_database_on_non_existing_dir() {
        let tempdir = TempDir::new();
        Database::<i32>::create(&tempdir.path.join("nested_folder")).unwrap();
    }

    #[test]
    fn creates_data_only_on_store() {
        let tempdir = TempDir::new();

        let database = Database::<DataType2>::create(&tempdir.path).unwrap();
        assert_eq!(tempdir.count_contained_items(), 1); // lock file

        database.store().unwrap();
        assert_eq!(tempdir.count_contained_items(), 2); // lock file and stored data
    }

    #[test]
    fn store_many_time_on_same_second_merges_data() {
        let tempdir = TempDir::new();

        let database = Database::<DataType3>::create(&tempdir.path).unwrap();
        assert_eq!(tempdir.count_contained_items(), 1); // lock file

        let path1 = database.store().unwrap();
        let path2 = database.store().unwrap();
        let path3 = database.store().unwrap();
        let path4 = database.store().unwrap();

        assert_eq!(tempdir.count_contained_items(), 2);
        assert_eq!(path1.parent(), path2.parent());
        assert_eq!(path2.parent(), path3.parent());
        assert_eq!(path3.parent(), path4.parent());

        assert!(path1.parent().unwrap().read_dir().unwrap().count() <= 3);
    }

    #[test]
    fn create_database_on_non_empty_dir() {
        let tempdir = TempDir::from_template(&[TemplateItem::File {
            path: "",
            name: "dummy.txt",
            content: "a great wall".as_bytes(),
        }]);

        let error = Database::<DataType3>::create(&tempdir.path);
        assert_eq!(format!("{:?}", error), "Err(AlreadyExist)");
    }

    #[test]
    fn store_default_and_open_twice() {
        let tempdir = TempDir::new();

        {
            let database = Database::<DataType3>::create(&tempdir.path).unwrap();
            database.store().unwrap();
        }
        {
            let mut database = Database::<DataType3>::open(&tempdir.path).unwrap();
            assert_eq!(database.data, Default::default());
            database.data = gen_data3();
            database.store().unwrap();
        }
        {
            let database = Database::<DataType3>::open(&tempdir.path).unwrap();
            assert_eq!(database.data, gen_data3());
        }
    }

    #[test]
    fn on_drop_data_is_stored() {
        let tempdir = TempDir::new();
        {
            let mut database = Database::<DataType2>::create(&tempdir.path).unwrap();
            database.data = gen_data2();
        }
        {
            let database = Database::<DataType2>::open(&tempdir.path).unwrap();
            assert_eq!(database.data, gen_data2());
        }
    }

    #[test]
    fn on_store_old_backups_are_deleted() {
        let tempdir = TempDir::new();
        // Setup Data

        let path = file::path_from_instant(&tempdir.path, &Instant::ymd(202, 11, 3));
        create_dir_all(path.parent().unwrap()).unwrap();
        file::serializer::save_data(&path, &5f32).unwrap();

        let path = file::path_from_instant(&tempdir.path, &Instant::ymd(2020, 11, 3));
        create_dir_all(path.parent().unwrap()).unwrap();
        file::serializer::save_data(&path, &4.3f32).unwrap();

        let path = file::path_from_instant(&tempdir.path, &Instant::ymd(2020, 11, 6));
        create_dir_all(path.parent().unwrap()).unwrap();
        file::serializer::save_data(&path, &4.2f32).unwrap();

        let path = file::path_from_instant(&tempdir.path, &Instant::ymd(2020, 11, 7));
        create_dir_all(path.parent().unwrap()).unwrap();
        file::serializer::save_data(&path, &4.1f32).unwrap();

        let path = file::path_from_instant(&tempdir.path, &Instant::ymd_hms(2020, 11, 7, 9, 0, 0));
        create_dir_all(path.parent().unwrap()).unwrap();
        file::serializer::save_data(&path, &4f32).unwrap();

        let path = file::path_from_instant(&tempdir.path, &Instant::ymd(2020, 12, 3));
        create_dir_all(path.parent().unwrap()).unwrap();
        file::serializer::save_data(&path, &3f32).unwrap();

        let path = file::path_from_instant(&tempdir.path, &Instant::now().truncate_time());
        create_dir_all(path.parent().unwrap()).unwrap();
        file::serializer::save_data(&path, &2f32).unwrap();

        // Load and cleanup Database
        {
            let mut database = Database::<f32>::open(&tempdir.path).unwrap();
            database.data = 1.;
        }

        // Check that the cleanup has worked
        let mut index = 0.;
        file::select_backup::<(), _>(&tempdir.path, |(path, _)| {
            index += 1.;
            assert_eq!(index, file::serializer::load_data::<f32>(&path).unwrap());
            None
        })
        .unwrap();
        assert_eq!(5., index);
    }

    #[test]
    fn rollback_on_empty_dir() {
        let tempdir = TempDir::new();
        let error = Database::<i32>::rollback(&tempdir.path);
        assert_eq!("Err(NotFound)", format!("{:?}", error));
    }

    #[test]
    fn rollback_lock_collision() {
        let tempdir = TempDir::new();
        let _database = Database::<i32>::create(&tempdir.path).unwrap();
        let error = Database::<i32>::rollback(&tempdir.path);
        assert_eq!("Err(Collision)", format!("{:?}", error));
    }

    #[test]
    fn rollback_on_non_corrupted_database() {
        let tempdir = TempDir::new();
        {
            let mut database = Database::<DataType3>::create(&tempdir.path).unwrap();
            database.data = gen_data3();
        }

        let database = Database::<DataType3>::rollback(&tempdir.path).unwrap();
        assert_eq!(gen_data3(), database.data);
    }

    #[test]
    fn rollback_on_corrupted_database() {
        let tempdir = TempDir::new();
        {
            let mut database = Database::<DataType3>::create(&tempdir.path).unwrap();
            database.data = gen_data3();
        }

        sleep_for(1100);

        // Insert corrupted data
        file::save_data::<u8>(&tempdir.path, &123).unwrap();

        let database = Database::<DataType3>::rollback(&tempdir.path).unwrap();
        assert_eq!(gen_data3(), database.data);
    }

    #[test]
    fn rollback_info_on_empty_dir() {
        let tempdir = TempDir::new();
        let error = Database::<i32>::rollback_info(&tempdir.path);
        assert_eq!("Err(NotFound)", format!("{:?}", error));
    }

    #[test]
    fn rollback_info_lock_collision() {
        let tempdir = TempDir::new();
        let _database = Database::<i32>::create(&tempdir.path).unwrap();
        let error = Database::<i32>::rollback_info(&tempdir.path);
        assert_eq!("Err(Collision)", format!("{:?}", error));
    }

    #[test]
    fn rollback_info_on_non_corrupted_database() {
        let tempdir = TempDir::new();

        let before = Instant::now();
        {
            let mut database = Database::<DataType3>::create(&tempdir.path).unwrap();
            database.data = gen_data3();
        }
        let after = Instant::now();

        let rollback_info = Database::<DataType3>::rollback_info(&tempdir.path).unwrap();
        assert_eq!(rollback_info.newest_instant, rollback_info.rollback_instant);
        assert!(before <= rollback_info.newest_instant);
        assert!(rollback_info.newest_instant <= after);
    }

    #[test]
    fn rollback_info_on_corrupted_database() {
        let tempdir = TempDir::new();

        let t0 = Instant::now();
        {
            let mut database = Database::<DataType3>::create(&tempdir.path).unwrap();
            database.data = gen_data3();
            database.store().unwrap();
        }
        let t1 = Instant::now();

        sleep_for(1100);

        // Insert corrupted data
        let t2 = Instant::now();
        file::save_data::<u8>(&tempdir.path, &123).unwrap();
        let t3 = Instant::now();

        let rollback_info = Database::<DataType3>::rollback_info(&tempdir.path).unwrap();
        assert_ne!(rollback_info.newest_instant, rollback_info.rollback_instant);

        assert!(t0 <= rollback_info.rollback_instant);
        assert!(t1 >= rollback_info.rollback_instant);

        assert!(t2 <= rollback_info.newest_instant);
        assert!(t3 >= rollback_info.newest_instant);
    }
}
