#[macro_use] extern crate rocket;

mod store_interface;
mod persisted_store;
mod memory_store;
mod events;
mod password;
mod user;
mod http_result;
mod api;

use structopt::StructOpt;
use std::path::PathBuf;
use store_interface::{ HashedPassword, Store };
use persisted_store::{ PersistedStore };
use user::{ User };
use serde::{ Serialize, Deserialize };
use rocket_contrib::json::Json;
use rocket::{State, http::CookieJar};
use http_result::{ HttpError, HttpResult };

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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let opts = Opts::from_args();
    println!("{:?}", opts);

    let store = PersistedStore::load(opts.database).await?;

    let mut rocket_config = rocket::config::Config::default();
    rocket_config.port = opts.port;

    rocket::custom(rocket_config)
        .manage(store)
        .mount("/", api::routes())
        .launch()
        .await?;

    Ok(())
}
