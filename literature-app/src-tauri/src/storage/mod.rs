pub mod index;
pub mod workspace;
pub mod paper;

use std::path::PathBuf;
use std::sync::Mutex;

pub struct AppState {
    pub root_dir: Mutex<Option<PathBuf>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            root_dir: Mutex::new(None),
        }
    }
}
