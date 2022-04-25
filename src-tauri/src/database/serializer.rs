use serde::{de::DeserializeOwned, Serialize};
use std::{fs::File, path::PathBuf};
use zstd::{decode_all, Encoder};

/// Converts data to binary,
/// compresses the binary using zstd
/// and saves it to a file
fn save_data<T: Serialize>(file_path: &PathBuf, data: &T) {
    let file = File::create(file_path).unwrap();
    let mut encoder = Encoder::new(file, 1).unwrap();
    bincode::serialize_into(&mut encoder, data).unwrap();
    encoder.finish().unwrap();
}

/// Intverts the serialization of `save_data`
fn load_data<T: DeserializeOwned>(file_path: &PathBuf) -> T {
    let file = File::open(file_path).unwrap();
    let decoded = decode_all(file).unwrap();
    bincode::deserialize(&decoded).unwrap()
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

            save_data(&path, &data);
            let loaded: Data = load_data(&path);

            println!("stored: {:?}", &data);
            println!("loaded: {:?}", &loaded);

            assert_eq!(data.len(), loaded.len());
            assert!(data.iter().zip(loaded.iter()).all(|(a, b)| a == b));
        }
        {
            type Data = Vec<u8>;
            let data: Data = vec![3, 1, 4, 1, 5, 9, 2, 7];

            save_data(&path, &data);
            let loaded: Data = load_data(&path);

            println!("stored: {:?}", &data);
            println!("loaded: {:?}", &loaded);

            assert_eq!(data.len(), loaded.len());
            assert!(data.iter().zip(loaded.iter()).all(|(a, b)| a == b));
        }
    }
}
