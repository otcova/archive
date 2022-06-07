mod datamodel;
mod lock;
mod serializer;

pub use super::{*, time::*};
pub use lock::*;
use serde::{de::DeserializeOwned, Serialize};
use std::{fs::create_dir_all, path::PathBuf};

pub fn load_newest<T: DeserializeOwned>(dir: &PathBuf) -> Result<T> {
    let newest = datamodel::select_backup(dir, |(path, _)| Some(path))?;
    if let Some(path) = newest {
        let data: T = serializer::load_data(&path)?;
        return Ok(data);
    }
    ErrorKind::NotFound.into()
}

/// Stores the data on the database as the newest backup
pub fn save_data<T: Serialize>(database_path: &PathBuf, data: &T) -> Result<PathBuf> {
    if !database_path.exists() {
        return ErrorKind::NotFound.into();
    }

    let path = datamodel::path_from_instant(&database_path, &Instant::now());
    create_dir_all(&path.parent().unwrap())?;
    serializer::save_data(&path, &data)?;
    Ok(path)
}

/// Loops over all database backups until it finds a non corrupted sample.
pub fn load_newest_noncurrupted<T: DeserializeOwned>(dir: &PathBuf) -> Result<T> {
    datamodel::select_backup(dir, |(path, _)| serializer::load_data(&path).ok())?
        .map_or_else(|| ErrorKind::NotFound.into(), |d| Ok(d))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;
    use std::{fs::File, io::Write, path::Path};

    #[test]
    fn load_newest_from_empty_dir() {
        let tempdir = TempDir::new();

        let result = load_newest::<DataType1>(&tempdir.path);
        assert_eq!(format!("{:?}", result), "Err(NotFound)");

        let result = load_newest::<DataType2>(&tempdir.path);
        assert_eq!(format!("{:?}", result), "Err(NotFound)");

        let result = load_newest::<DataType3>(&tempdir.path);
        assert_eq!(format!("{:?}", result), "Err(NotFound)");
    }

    #[test]
    fn load_newest_noncorrupted_from_empty_dir() {
        let tempdir = TempDir::new();

        let result = load_newest_noncurrupted::<DataType1>(&tempdir.path);
        assert_eq!(format!("{:?}", result), "Err(NotFound)");

        let result = load_newest_noncurrupted::<DataType2>(&tempdir.path);
        assert_eq!(format!("{:?}", result), "Err(NotFound)");

        let result = load_newest_noncurrupted::<DataType3>(&tempdir.path);
        assert_eq!(format!("{:?}", result), "Err(NotFound)");
    }

    /// This test might fail if
    /// you are working in new year
    /// and you are very unlucky
    #[test]
    fn save_data_creates_year_folder() {
        let tempdir = TempDir::new();

        let now = Instant::now();
        save_data(&tempdir.path, &gen_data1()).unwrap();

        let year_folder = Path::new(&tempdir.path).join(now.year().to_string());
        assert!(year_folder.exists());
    }

    #[test]
    fn saved_data_can_be_deserialized() {
        let tempdir = TempDir::new();

        let saved_data = gen_data3();
        let path = save_data(&tempdir.path, &saved_data).unwrap();
        let loaded_data = serializer::load_data::<DataType3>(&path).unwrap();

        assert_eq!(saved_data, loaded_data);
    }

    #[test]
    fn multiple_save_data_calls_on_same_second_overlap() {
        let tempdir = TempDir::new();

        let saved_data_1 = gen_data1();
        let saved_data_2 = gen_data2();
        let saved_data_3 = gen_data3();

        let start = std::time::Instant::now();

        let path_1 = save_data(&tempdir.path, &saved_data_1).unwrap();
        let path_2 = save_data(&tempdir.path, &saved_data_2).unwrap();
        let path_3 = save_data(&tempdir.path, &saved_data_3).unwrap();

        let end = std::time::Instant::now();
        let duration = (end - start).as_secs_f32();

        assert!(duration < 1., "save_data is to slow");

        // If the tree calls have been done in less than a second
        // at least two of them are in the same second
        // so at least two of them have overlaped

        // Check for override
        assert!(path_1 == path_2 || path_2 == path_3);

        // Check that overrided content is correct
        if path_2 == path_3 {
            let loaded_data_3 = serializer::load_data::<DataType3>(&path_3).unwrap();
            assert_eq!(saved_data_3, loaded_data_3);
        } else if path_1 == path_2 {
            let loaded_data_2 = serializer::load_data::<DataType2>(&path_2).unwrap();
            assert_eq!(saved_data_2, loaded_data_2);
        }
    }

    #[test]
    fn load_newest_use_case() {
        let tempdir = TempDir::new();

        let saved_data_1 = gen_data1();
        let saved_data_2 = gen_data2();
        let saved_data_3 = gen_data3();

        save_data(&tempdir.path, &saved_data_1).unwrap();
        assert_eq!(
            load_newest::<DataType1>(&tempdir.path).unwrap(),
            saved_data_1
        );

        save_data(&tempdir.path, &saved_data_2).unwrap();
        assert_eq!(
            load_newest::<DataType2>(&tempdir.path).unwrap(),
            saved_data_2
        );

        save_data(&tempdir.path, &saved_data_3).unwrap();
        assert_eq!(
            load_newest::<DataType3>(&tempdir.path).unwrap(),
            saved_data_3
        );
    }

    #[test]
    fn load_newest_when_data_is_corrupted() {
        let tempdir = TempDir::new();

        // Create Data
        save_data(&tempdir.path, &gen_data2()).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(1100));
        let path = save_data(&tempdir.path, &gen_data2()).unwrap();

        // Corrupt data
        File::create(path)
            .unwrap()
            .write_all(&[2, 1, 5, 0])
            .unwrap();

        // Read data
        let error = load_newest::<DataType3>(&tempdir.path);
        assert_eq!(format!("{:?}", error), "Err(DataIsCorrupted)");
    }

    #[test]
    fn load_newest_noncorrupted_data_use_case() {
        let tempdir = TempDir::new();

        let saved_data = gen_data3();

        // Create Data
        save_data(&tempdir.path, &saved_data).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(1100));
        let path = save_data(&tempdir.path, &saved_data).unwrap();

        // Corrupt data
        File::create(path)
            .unwrap()
            .write_all(&[2, 1, 5, 0])
            .unwrap();

        // Read data
        let data = load_newest_noncurrupted::<DataType3>(&tempdir.path).unwrap();
        assert_eq!(data, saved_data);
    }
}
