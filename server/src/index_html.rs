use rocket::response::NamedFile;
use std::path::{ PathBuf };
use rocket::{ State };
use crate::state;

/// Serve up
pub fn index_route() -> Vec<rocket::Route> {
    routes![index]
}

// rank of 1000 means it'll be called after practically anything else is tried.
#[get("/<_path..>", rank = 1000)]
async fn index(state: State<'_, state::State>, _path: PathBuf) -> Option<NamedFile> {
    match &state.static_files {
        Some(static_file_root) => {
            NamedFile::open(static_file_root.join("index.html")).await.ok()
        },
        None => {
            None
        }
    }
}