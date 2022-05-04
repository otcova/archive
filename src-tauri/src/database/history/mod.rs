mod time;

use self::time::Instant;
use super::{
    error::{ErrorKind, Result},
    serializer,
};
use serde::Serialize;
use std::{fs::create_dir, path::PathBuf};

pub fn load_data<T>(_dir: &PathBuf) -> Result<T> {
    Err(ErrorKind::NotFound.into())
}

pub fn save_data<T: Serialize>(database_path: &PathBuf, data: T) -> Result<PathBuf> {
    if !database_path.exists() {
        return Err(ErrorKind::NotFound.into());
    }

    let now = Instant::now();

    let year_folder = database_path.join(now.year().to_string());
    create_dir(&year_folder)?;

    let file_path = year_folder.join(now.str()).with_extension("bin");
    serializer::save_data(&file_path, &data)?;
    Ok(file_path)
}

pub fn load_timeline() {}

pub fn recover_data<T>(_dir: &PathBuf) {}

#[cfg(test)]
mod tests {
    use super::time::Instant;
    use crate::database::{history as h, serializer, test_utils::TempDir};
    use serde::{Serialize, Deserialize};
    use std::path::Path;

    type DataType1 = usize;
    #[derive(Debug, PartialEq, Eq, Serialize)]
    struct DataType2(Vec<usize>);

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
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

    /// This test might fail if
    /// you are working in new year
    /// and you are very unlucky
    #[test]
    fn save_data_creates_year_folder() {
        let tempdir = TempDir::new();

        let now = Instant::now();
        h::save_data(&tempdir.path, gen_data1()).unwrap();

        let year_folder = Path::new(&tempdir.path).join(now.year().to_string());
        assert!(year_folder.exists());
    }

    #[test]
    fn saved_data_can_be_deserialized() {
        let tempdir = TempDir::new();
 
        let saved_data = gen_data3();
        let path = h::save_data(&tempdir.path, &saved_data).unwrap();
        let loaded_data = serializer::load_data::<DataType3>(&path).unwrap();
        
        assert_eq!(saved_data, loaded_data);
 
    }
}
