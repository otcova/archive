use serde::{Deserialize, Serialize};
use std::io::prelude::*;
use std::thread::sleep;
use std::time::Duration;
use std::{
    fs::{create_dir_all, File},
    path::PathBuf,
};

fn gen_id() -> usize {
    use std::sync::atomic::{AtomicUsize, Ordering};
    static ID: AtomicUsize = AtomicUsize::new(0);
    ID.fetch_add(1, Ordering::SeqCst)
}

fn test_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn gen_unique_test_dir() -> PathBuf {
    test_dir().join(format!("test-{}", gen_id()))
}

pub enum TemplateItem<'a> {
    File {
        path: &'a str,
        name: &'a str,
        content: &'a [u8],
    },
}

pub struct TempDir {
    pub path: PathBuf,
}

impl TempDir {
    pub fn new() -> Self {
        let path = gen_unique_test_dir();
        std::fs::create_dir_all(&path).unwrap();
        Self { path }
    }
    pub fn from_template(template: &[TemplateItem]) -> Self {
        let dir = Self::new();
        for item in template {
            match item {
                TemplateItem::File {
                    path,
                    name,
                    content,
                } => {
                    let absolute_path = dir.path.join(path.replace("/", "\\"));
                    create_dir_all(&absolute_path).unwrap();
                    let mut file = File::create(&absolute_path.join(name)).unwrap();
                    if content.len() > 0 {
                        file.write_all(content).unwrap();
                    }
                }
            }
        }
        dir
    }
    pub fn is_empty(&self) -> bool {
        self.path.read_dir().unwrap().next().is_none()
    }
    pub fn count_contained_items(&self) -> usize {
        self.path.read_dir().unwrap().count()
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        std::fs::remove_dir_all(&self.path).unwrap();
    }
}

pub type DataType1 = usize;
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct DataType2(Vec<usize>);

#[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataType3 {
    name: String,
    matrix: Vec<Vec<f32>>,
}

pub fn gen_data1() -> DataType1 {
    92810
}
pub fn gen_data2() -> DataType2 {
    DataType2(vec![92810, 213, 1, 321312, 4, 0])
}
pub fn gen_data3() -> DataType3 {
    DataType3 {
        name: "Some persone".into(),
        matrix: vec![vec![2., 0., 1.], vec![0., 1e10, -5.], vec![1.3, 0.3, -1.]],
    }
}

pub fn sleep_for(millis: u64) {
    sleep(Duration::from_millis(millis));
}
