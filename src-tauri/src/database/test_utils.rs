use std::path::PathBuf;

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

pub struct TempDir {
    pub path: PathBuf,
}

impl TempDir {
    pub fn new() -> Self {
        let path = gen_unique_test_dir();
        std::fs::create_dir_all(&path).unwrap();
        Self { path }
    }
    pub fn is_empty(&self) -> bool {
        self.path.read_dir().unwrap().next().is_none()
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        std::fs::remove_dir_all(&self.path).unwrap();
    }
}
