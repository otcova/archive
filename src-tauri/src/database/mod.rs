#![allow(dead_code)]

mod backup;
mod error;
mod lock;
mod file_serializer;

use self::{
    error::{ErrorKind, Result},
    lock::Lock,
};
use serde::{de::DeserializeOwned, Serialize};
use std::path::PathBuf;

#[derive(Debug)]
pub struct Database<T: Default + DeserializeOwned + Serialize> {
    lock: Lock,
    pub data: T,
    path: PathBuf,
}

impl<T> Database<T>
where
    T: Default + DeserializeOwned + Serialize,
{
    pub fn open(path: &PathBuf) -> Result<Self> {
        let lock = Lock::directory(&path)?;
        let data: T = backup::load_newest(&path)?;
        Ok(Self {
            lock,
            data,
            path: path.clone(),
        })
    }

    pub fn create(path: &PathBuf) -> Result<Self> {
        if !path.read_dir().unwrap().next().is_none() {
            return ErrorKind::AlreadyExist.into();
        }

        let lock = Lock::directory(&path)?;

        let data: T = Default::default();
        Ok(Self {
            lock,
            data,
            path: path.clone(),
        })
    }

    pub fn store(&self) -> Result<PathBuf> {
        backup::save_data(&self.path, &self.data)
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
    use crate::test_utils::*;
    use super::Database;
    
    #[test]
    fn open_database_on_empty_dir() {
        let tempdir = TempDir::new();
        let error = Database::<i32>::open(&tempdir.path);
        assert_eq!(format!("{:?}", error), "Err(NotFound)");
    }

    #[test]
    fn create_database_on_empty_dir() {
        let tempdir = TempDir::new();
        let database = Database::<i32>::create(&tempdir.path);
        assert!(database.is_ok());
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

        assert!(path1.parent().unwrap().read_dir().unwrap().count() < 3);
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
}
