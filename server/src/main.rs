mod store_interface;
mod persisted_store;
mod memory_store;
mod sessions;
mod events;
mod password;

use structopt::StructOpt;
use std::path::PathBuf;
use std::sync::Arc;
use warp::Filter;
use store_interface::{ HashedPassword, Store };
use persisted_store::{ PersistedStore, PersistedStoreError };
use serde::{ Serialize, Deserialize };
use std::convert::Infallible;

#[derive(Debug,Clone,StructOpt)]
struct Opts {
    /// Which port will the app run on
    #[structopt(long,short)]
    port: u16,
    /// Where do the client files live
    #[structopt(long,short,parse(from_os_str))]
    static_files: PathBuf,
    /// Where does the database live
    #[structopt(long,short)]
    database: PathBuf
}

// #[tokio::main]
// async fn main() -> anyhow::Result<()> {
//     env_logger::init();

//     let opts = Opts::from_args();
//     println!("{:?}", opts);

//     let store = PersistedStore::load(opts.database).await?;

//     let api_routes = warp::path("api").and(make_api_routes(store));
//     let static_files = warp::fs::dir(opts.static_files);

//     let routes = static_files.or(api_routes);

//     warp::serve(routes).run(([127, 0, 0, 1], opts.port)).await;

//     Ok(())
// }

// fn make_api_routes(store: PersistedStore) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
//     let store = Arc::new(store);
//     let with_store = warp::any().map(move || store.clone());

//     #[derive(Serialize,Clone,Copy)]
//     struct Empty {}

//     #[derive(Deserialize)]
//     struct UpsertUserInput {
//         username: String,
//         password: String
//     }

//     let upsert_user = warp::path("upsert_user")
//         .and(with_store)
//         .and(warp::filters::body::json())
//         .and_then(|store: Arc<PersistedStore>, body: UpsertUserInput| async move {
//             let hashed_pw = HashedPassword::from_plain_password(&body.password);
//             store.upsert_user(body.username, hashed_pw)?;
//             Ok::<_,Infallible>(warp::reply::json(&Empty {}))
//         });

//     let routes = upsert_user.recover(|e| {
//         Ok(warp::reply::with_status(json, status_code))
//     });

//     routes
// }