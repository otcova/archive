use super::time::Instant;
use crate::database::error::Result;
use std::path::{Path, PathBuf};

pub fn file_path_of(dir: &PathBuf, instant: &Instant) -> PathBuf {
    dir.join(instant.year().to_string())
        .join(instant.str() + ".bin")
}

/// Concats all files and folders of a directory
/// filter_map all the folder names and file stem
/// and sorts the content (greater to smaller)
fn scan_folder<T, F>(dir: &PathBuf, mut filter_map: F) -> Result<Vec<(PathBuf, T)>>
where
    T: PartialOrd,
    F: FnMut(&str) -> Option<T>,
{
    let mut content: Vec<(PathBuf, T)> = dir
        .read_dir()?
        .flatten()
        .filter_map(|dir| {
            Some((
                dir.path(),
                filter_map(Path::new(&dir.file_name()).file_stem()?.to_str()?)?,
            ))
        })
        .collect();
    content.sort_unstable_by(|a, b| a.1.partial_cmp(&b.1).unwrap().reverse());
    Ok(content)
}

/// Reads database tree and returns the path of the selected file
pub fn select_database_backup<T, F>(dir: &PathBuf, select: F) -> Result<Option<T>>
where
    F: FnMut((PathBuf, Instant)) -> Option<T>,
{
    Ok(
        scan_folder::<i32, _>(dir, |folder_year| folder_year.parse().ok())?
            .iter()
            .flat_map(|(path, _)| scan_folder(&path, |file_time| Instant::from_utc(file_time).ok()))
            .flatten()
            .find_map(select),
    )
}

#[cfg(test)]
mod tests {
    use crate::database::{
        backup::{datamodel::*, time::Instant},
        test_utils::*,
    };

    #[test]
    fn file_path_use_case() {
        let tempdir = TempDir::new();
        let instant_1 = Instant::from_utc("2021_05_06 13_52_05").unwrap();
        let instant_2 = Instant::from_utc("2021_05_06 13_51_05").unwrap();
        let instant_3 = Instant::from_utc("2020_05_06 13_51_05").unwrap();
        assert_eq!(
            file_path_of(&tempdir.path, &instant_1),
            tempdir.path.join("2021").join(instant_1.str() + ".bin")
        );
        assert_eq!(
            file_path_of(&tempdir.path, &instant_2),
            tempdir.path.join("2021").join(instant_2.str() + ".bin")
        );
        assert_eq!(
            file_path_of(&tempdir.path, &instant_3),
            tempdir.path.join("2020").join(instant_3.str() + ".bin")
        );
    }

    #[test]
    fn select_database_backup_use_case() {
        let instant_1 = Instant::from_utc("2021_05_06 13_52_05").unwrap();
        let instant_2 = Instant::from_utc("2021_05_06 13_51_05").unwrap();
        let instant_3 = Instant::from_utc("2020_05_06 13_51_05").unwrap();

        let tempdir = TempDir::from_template(&[
            TemplateItem::File {
                path: "2021",
                name: "2021_05_07 13h 51m 05s.bin",
                content: &[],
            },
            TemplateItem::File {
                path: "2021",
                name: (instant_1.str() + ".bin").as_str(),
                content: &[],
            },
            TemplateItem::File {
                path: "2021",
                name: (instant_2.str() + ".bin").as_str(),
                content: &[],
            },
            TemplateItem::File {
                path: "2020",
                name: (instant_3.str() + ".bin").as_str(),
                content: &[],
            },
        ]);

        // Select first
        assert_eq!(
            select_database_backup(&tempdir.path, |(_, instant)| {
                assert!(instant == instant_1, "Newest backup is not first");
                Some(123)
            })
            .unwrap()
            .unwrap(),
            123
        );

        // Select second
        let mut index = 0;
        assert_eq!(
            select_database_backup(&tempdir.path, |(_, instant)| {
                if instant == instant_2 {
                    assert!(index == 1, "Backups are not in order");
                    return Some("abAca");
                }
                assert!(index < 1, "Backups are not in order");
                index += 1;
                None
            })
            .unwrap()
            .unwrap(),
            "abAca"
        );

        // Select
        let mut index = 0;
        assert_eq!(
            select_database_backup(&tempdir.path, |(_, instant)| {
                if instant == instant_3 {
                    assert!(index == 2, "Backups are not in order");
                    return Some(instant);
                }
                assert!(index < 2, "Backups are not in order");
                index += 1;
                None
            })
            .unwrap()
            .unwrap(),
            instant_3
        );
    }
}
