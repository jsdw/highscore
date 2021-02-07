#[macro_use] extern crate rocket;

mod store_interface;
mod persisted_store;
mod memory_store;
mod events;
mod password;
mod user;
mod http_result;
mod api;

use anyhow::Context;
use structopt::StructOpt;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt};
use std::{path::PathBuf};
use persisted_store::{ PersistedStore };
use store_interface::{HashedPassword, Store};

#[derive(Debug,Clone,StructOpt)]
enum Opts {
    AddUser(AddUserOpts),
    Serve(ServeOpts)
}

#[derive(Debug,Clone,StructOpt)]
struct AddUserOpts {
    /// Where does the database live
    #[structopt(long,short)]
    database: PathBuf
}

#[derive(Debug,Clone,StructOpt)]
struct ServeOpts {
    /// Which port will the app run on
    #[structopt(long,short,default_value="8080")]
    port: u16,
    /// Where do the client files live
    #[structopt(long,short,parse(from_os_str))]
    static_files: Option<PathBuf>,
    /// Where does the database live
    #[structopt(long,short)]
    database: PathBuf
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let opts = Opts::from_args();
    println!("{:?}", opts);

    match opts {
        Opts::AddUser(opts) => add_user(opts).await,
        Opts::Serve(opts) => serve(opts).await
    }
}

/// Add a new user to a database, creating the file if not exists.
async fn add_user(opts: AddUserOpts) -> anyhow::Result<()> {
    let username = prompt_for_input("Username: ").await?;
    let username = username.trim_end_matches('\n');
    let password = prompt_for_hidden_input("Password: ").await?;
    let password = password.trim_end_matches('\n');
    let hashed_password = HashedPassword::from_plain_password(&password);

    let store = PersistedStore::load(opts.database).await?;
    store.upsert_user(username.to_owned(), hashed_password).await?;
    store.flush_to_disk().await?;

    println!("User {} added", username);
    Ok(())
}

/// Serve the API somewhere.
async fn serve(opts: ServeOpts) -> anyhow::Result<()> {
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

/// Prompt for input from stdin
async fn prompt_for_input(msg: &str) -> anyhow::Result<String> {
    let mut stdout = tokio::io::stdout();
    stdout.write_all(msg.as_bytes())
        .await
        .with_context(|| format!("Could not write to stdout"))?;
    stdout.flush()
        .await
        .with_context(|| format!("Could not write to stdout (2)"))?;
    let mut username = String::new();
    tokio::io::BufReader::new(tokio::io::stdin()).read_line(&mut username)
        .await
        .with_context(|| format!("Failed to read username from stdin"))?;
    Ok(username)
}

/// Prompt for password-like input (input is hidden)
async fn prompt_for_hidden_input(msg: &str) -> anyhow::Result<String> {
    let msg = msg.to_owned();
    tokio::task::spawn_blocking(move ||
        rpassword::prompt_password_stderr(&msg)
            .with_context(|| format!("Failed to read password from stdin"))
    ).await?
}