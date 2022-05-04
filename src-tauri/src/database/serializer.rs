use serde::{de::DeserializeOwned, Serialize};
use std::{fs::File, path::PathBuf};
use zstd::{decode_all, Encoder};
use super::error::{Result, ErrorKind};

/// Converts data to binary,
/// compresses the binary using zstd
/// and saves it to a file
pub fn save_data<T: Serialize>(file_path: &PathBuf, data: &T) -> Result<()> {
    let file = File::create(file_path)?;
    let mut encoder = Encoder::new(file, 1)?;
    bincode::serialize_into(&mut encoder, data).unwrap();
    encoder.finish()?;
    Ok(())
}

/// Intverts the serialization of `save_data`
pub fn load_data<T: DeserializeOwned>(file_path: &PathBuf) -> Result<T> {
    let file = File::open(file_path)?;
    let file_bytes = decode_all(file)?;
    if let Ok(data) = bincode::deserialize(&file_bytes) {
        return Ok(data);
    }
    ErrorKind::DataIsCorrupted.into()
}

#[cfg(test)]
mod tests {
    use super::{load_data, save_data};
    use crate::database::test_utils::TempDir;

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
}
