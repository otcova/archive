mod time;

use self::time::Instant;
use super::{
    error::{ErrorKind, Result},
    serializer,
};
use serde::Serialize;
use std::{fs::create_dir_all, path::PathBuf};

pub fn load_data<T>(_dir: &PathBuf) -> Result<T> {
    Err(ErrorKind::NotFound.into())
}

pub fn save_data<T: Serialize>(database_path: &PathBuf, data: T) -> Result<PathBuf> {
    if !database_path.exists() {
        return Err(ErrorKind::NotFound.into());
    }
    let now = Instant::now();

    let year_folder = database_path.join(now.year().to_string());
    create_dir_all(&year_folder)?;

    let file_path = year_folder.join(now.str()).with_extension("bin");
    serializer::save_data(&file_path, &data)?;
    Ok(file_path)
}

/// Reads database tree (folders and files)
/// and returns the path of an specified instant or older
fn get_newest_file_path_since(
    _dir: &PathBuf,
    _since_instant: &Instant,
) -> Result<(PathBuf, Instant)> {
    Err(ErrorKind::NotFound.into())
}

pub fn recover_data<T>(_dir: &PathBuf) {}

#[cfg(test)]
mod tests {
    use super::time::Instant;
    use crate::database::{
        history::{self as h, get_newest_file_path_since},
        serializer,
        test_utils::{TempDir, TemplateItem},
    };
    use serde::{Deserialize, Serialize};
    use std::{path::Path};

    type DataType1 = usize;
    #[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
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

    #[test]
    fn multiple_save_data_calls_on_same_second_overlap() {
        let tempdir = TempDir::new();

        let saved_data_1 = gen_data1();
        let saved_data_2 = gen_data2();
        let saved_data_3 = gen_data3();

        let start = std::time::Instant::now();

        let path_1 = h::save_data(&tempdir.path, &saved_data_1).unwrap();
        let path_2 = h::save_data(&tempdir.path, &saved_data_2).unwrap();
        let path_3 = h::save_data(&tempdir.path, &saved_data_3).unwrap();

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
    fn get_newest_file_path_since_use_case() {
        let tempdir = TempDir::from_template(&[
            TemplateItem::File {
                path: "2021",
                name: "2021_05_07 13h 51m 05s.bin",
                content: &[],
            },
            TemplateItem::File {
                path: "2021",
                name: "2021_05_06 13_51_05.bin",
                content: &[],
            },
            TemplateItem::File {
                path: "2021",
                name: "2021_05_06 13_52_05.bin",
                content: &[],
            },
            TemplateItem::File {
                path: "2020",
                name: "2020_05_06 13_51_05.bin",
                content: &[],
            },
        ]);

        let (path_a, instant_a) = get_newest_file_path_since(&tempdir.path, &Instant::now()).unwrap();
        assert_eq!(path_a, tempdir.path.join("2021//2021_05_06 13_51_05.bin"));
        assert_eq!(instant_a, Instant::from_utc("2021_05_06 13_51_05").unwrap());
        
        let (path_b, instant_b) = get_newest_file_path_since(&tempdir.path, &Instant::now()).unwrap();
        assert_eq!(path_b, tempdir.path.join("2021//2021_05_06 13_52_05.bin"));
        assert_eq!(instant_b, Instant::from_utc("2021_05_06 13_52_05").unwrap());
        
        let (path_c, instant_c) = get_newest_file_path_since(&tempdir.path, &Instant::now()).unwrap();
        assert_eq!(path_c, tempdir.path.join("2020//2020_05_06 13_51_05.bin"));
        assert_eq!(instant_c, Instant::from_utc("2020_05_06 13_51_05").unwrap());
    }
}
