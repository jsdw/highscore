use crate::store_interface;
use std::path::PathBuf;

pub struct State {
    pub store: Box<dyn store_interface::Store + Send + Sync + 'static>,
    pub static_files: Option<PathBuf>
}