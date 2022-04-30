mod time;

use super::error::{ErrorKind, Result};
use std::path::PathBuf;

pub struct Date(String);

pub fn load_data<T>(_dir: &PathBuf) -> Result<T> {
    Err(ErrorKind::NotFound.into())
}

pub fn save_data<T>(_dir: &PathBuf, _data: T) -> Result<T> {
    Err(ErrorKind::NotFound.into())
}

pub fn load_timeline() {
    
}

pub fn recover_data<T>(_dir: &PathBuf) {
    
}

#[cfg(test)]
mod tests {
    use crate::database::{error::ErrorKind, history as h, test_utils::TempDir};

    type DataType1 = usize;
    #[derive(Debug, PartialEq, Eq)]
    struct DataType2(Vec<usize>);

    #[derive(Debug, PartialEq)]
    struct DataType3 {
        name: String,
        matrix: Vec<Vec<f32>>,
    }

    fn gen_data1() -> DataType1 {
        92810
    }
    fn gen_data2() -> DataType2 {
        DataType2(vec![92810, 213, 1, 321312, 4, 0])
    }
    fn gen_data3() -> DataType3 {
        DataType3 {
            name: String::from("Some persone"),
            matrix: vec![vec![2., 0., 1.], vec![0., 1e10, -5.], vec![1.3, 0.3, -1.]],
        }
    }

    #[test]
    fn load_data_from_empty_dir() {
        let tempdir = TempDir::new();

        let result = h::load_data::<DataType1>(&tempdir.path);
        assert_eq!(format!("{:?}", result), "Err(NotFound)");

        let result = h::load_data::<DataType2>(&tempdir.path);
        assert_eq!(format!("{:?}", result), "Err(NotFound)");

        let result = h::load_data::<DataType3>(&tempdir.path);
        assert_eq!(format!("{:?}", result), "Err(NotFound)");
    }

    #[test]
    fn create_data_on_empty_dir() {
        let dir1 = TempDir::new();
        assert!(h::save_data(&dir1.path, gen_data1()).is_ok());
        assert!(!dir1.is_empty());

        let dir2 = TempDir::new();
        assert!(h::save_data(&dir2.path, gen_data2()).is_ok());
        assert!(!dir2.is_empty());

        let dir3 = TempDir::new();
        assert!(h::save_data(&dir3.path, gen_data3()).is_ok());
        assert!(!dir3.is_empty());
    }

    #[test]
    fn create_data_on_non_empty_dir() {
        let tempdir = TempDir::new();
        assert!(h::save_data(&tempdir.path, gen_data1()).is_ok());

        let err1 = h::save_data(&tempdir.path, gen_data1()).unwrap_err();
        assert!(matches!(err1.as_ref(), ErrorKind::AlreadyExist));
        
        let err2 = h::save_data(&tempdir.path, gen_data2()).unwrap_err();
        assert!(matches!(err2.as_ref(), ErrorKind::AlreadyExist));
        
        let err3 = h::save_data(&tempdir.path, gen_data3()).unwrap_err();
        assert!(matches!(err3.as_ref(), ErrorKind::AlreadyExist));
    }
}