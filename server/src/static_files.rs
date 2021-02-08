use std::path::{ PathBuf };
use include_dir::Dir;
use rocket::http::ContentType;
use rocket::Response;

pub static PROJECT_DIR: Dir = include_dir::include_dir!("../client/public");

/// Serve up embedded static files
pub fn static_files_route() -> Vec<rocket::Route> {
    routes![static_files_root, static_files]
}

#[get("/")]
async fn static_files_root() -> Option<Response<'static>> {
    serve_file(PathBuf::from("index.html"))
}

#[get("/<path..>")]
async fn static_files(path: PathBuf) -> Option<Response<'static>> {
    serve_file(path)
}

fn serve_file(path: PathBuf) -> Option<Response<'static>> {
    let ext = path.extension()?.to_string_lossy();
    let content_type = content_type(&ext)?;
    let file = PROJECT_DIR.get_file(path)?.contents();
    let res = Response::build()
        .header(content_type)
        .sized_body(file.len(), std::io::Cursor::new(file))
        .finalize();
    Some(res)
}

fn content_type(ext: &str) -> Option<ContentType> {
    ContentType::from_extension(ext)
}