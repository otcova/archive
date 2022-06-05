use super::*;
use serde::{de::DeserializeOwned, Serialize};
use std::{
    fs::{self, File},
    path::PathBuf,
};
use zstd::{decode_all, Encoder};

/// Converts data to binary,
/// compresses the binary using zstd
/// and saves it to a file
pub fn save_data<T: Serialize>(path: &PathBuf, data: &T) -> Result<()> {
    let extension = String::from(path.extension().unwrap_or_default().to_string_lossy());
    let tmp_path = path.with_extension(extension + ".tmp");

    {
        let file = File::create(&tmp_path)?;

        let mut encoder = Encoder::new(file, 1)?;
        bincode::serialize_into(&mut encoder, data).unwrap();
        encoder.finish()?;
    }

    fs::rename(tmp_path, path)?;
    Ok(())
}

/// Intverts the serialization of `save_data`
pub fn load_data<T: DeserializeOwned>(file_path: &PathBuf) -> Result<T> {
    let file = File::open(file_path)?;
    if let Ok(file_bytes) = decode_all(file) {
        if let Ok(data) = bincode::deserialize(&file_bytes) {
            return Ok(data);
        }
    }
    ErrorKind::DataIsCorrupted.into()
}

#[cfg(test)]
mod tests {
    use super::{load_data, save_data};
    use crate::test_utils::TempDir;

    #[test]
    fn serialize_and_deserilize() {
        let tempdir = TempDir::new();
        let path = tempdir.path.join("data");
        {
            type Data = [i32; 4];
            let data: Data = [1, 2, 3, 4];

            save_data(&path, &data).unwrap();
            let loaded: Data = load_data(&path).unwrap();

            println!("stored: {:?}", &data);
            println!("loaded: {:?}", &loaded);

            assert_eq!(data.len(), loaded.len());
            assert!(data.iter().zip(loaded.iter()).all(|(a, b)| a == b));
        }
        {
            type Data = Vec<u8>;
            let data: Data = vec![3, 1, 4, 1, 5, 9, 2, 7];

            save_data(&path, &data).unwrap();
            let loaded: Data = load_data(&path).unwrap();

            println!("stored: {:?}", &data);
            println!("loaded: {:?}", &loaded);

            assert_eq!(data.len(), loaded.len());
            assert!(data.iter().zip(loaded.iter()).all(|(a, b)| a == b));
        }
    }

    #[test]
    fn load_data_of_random_file_throws_corrupted_data() {
        let tempdir = TempDir::new();
        let path = tempdir.path.join("data");

        save_data(&path, &[1u8, 2, 3]).unwrap();
        let result = load_data::<i64>(&path);

        assert_eq!(format!("{:?}", result), "Err(DataIsCorrupted)");
    }

    #[test]
    fn save_data_overwrites_content() {
        let tempdir = TempDir::new();
        let path = tempdir.path.join("data");

        type Data = Vec<u8>;
        let data_a: Data = vec![3, 1, 4, 1, 5, 9, 2, 7];
        let data_b: Data = vec![6, 5, 2, 1];

        save_data(&path, &data_a).unwrap();
        save_data(&path, &data_b).unwrap();

        let loaded: Data = load_data(&path).unwrap();

        assert_eq!(data_b.len(), loaded.len());
        assert!(data_b.iter().zip(loaded.iter()).all(|(a, b)| a == b));
    }
}
