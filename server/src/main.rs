#[macro_use] extern crate rocket;

mod store_interface;
mod persisted_store;
mod memory_store;
mod events;
mod password;
mod user;
mod http_result;
mod api;
mod state;
mod static_files;

use anyhow::Context;
use structopt::StructOpt;
use std::{path::PathBuf};
use persisted_store::{ PersistedStore };
use store_interface::{HashedPassword, Store};

#[derive(Debug,Clone,StructOpt)]
enum Opts {
    /// Add a user to a highscore database
    Users(Users),
    /// Run the highscore server
    Serve(ServeOpts)
}

#[derive(Debug,Clone,StructOpt)]
enum Users {
    /// Add a new user
    Add(NamedUserOpts),
    /// List users
    List(UserOpts),
    /// Remove a user
    Remove(NamedUserOpts)
}

#[derive(Debug,Clone,StructOpt)]
struct NamedUserOpts {
    /// The username
    username: String,
    #[structopt(flatten)]
    opts: UserOpts
}

#[derive(Debug,Clone,StructOpt)]
struct UserOpts {
    /// Where does the database live
    #[structopt(long,short)]
    database: PathBuf
}

#[derive(Debug,Clone,StructOpt)]
struct ServeOpts {
    /// Address to serve on
    #[structopt(long,short,default_value="127.0.0.1")]
    address: std::net::IpAddr,
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

    match opts {
        Opts::Users(Users::Add(opts)) => add_user(opts).await,
        Opts::Users(Users::List(opts)) => list_users(opts).await,
        Opts::Users(Users::Remove(opts)) => remove_user(opts).await,
        Opts::Serve(opts) => serve(opts).await
    }
}

/// Add a new user to a database, creating the file if not exists.
async fn add_user(opts: NamedUserOpts) -> anyhow::Result<()> {
    let username = opts.username;
    let password = prompt_for_hidden_input("Password: ").await?;
    let password = password.trim_end_matches('\n');

    let hashed_password = HashedPassword::from_plain_password(&password);
    let store = PersistedStore::load(opts.opts.database).await?;
    store.upsert_user(username.clone(), hashed_password).await?;
    store.flush_to_disk().await?;

    println!("User {} added.", username);
    Ok(())
}

/// List users in the database
async fn list_users(opts: UserOpts) -> anyhow::Result<()> {
    let store = PersistedStore::load(opts.database).await?;
    let mut users = store.users().await?;
    users.sort();
    for user in users {
        println!("{}", user);
    }
    Ok(())
}

/// Remove a user from the database.
async fn remove_user(opts: NamedUserOpts) -> anyhow::Result<()> {
    let store = PersistedStore::load(opts.opts.database).await?;
    let username = opts.username;
    store.delete_user(&username).await?;
    store.flush_to_disk().await?;

    println!("User {} removed.", username);
    Ok(())
}

/// Serve the API somewhere.
async fn serve(opts: ServeOpts) -> anyhow::Result<()> {
    println!("{:#?}", opts);
    let store = PersistedStore::load(opts.database).await?;

    let mut rocket_config = rocket::config::Config::default();
    rocket_config.port = opts.port;
    rocket_config.address = opts.address;
    // Generate a new key each time; this will invalidate existing sessions
    // on a restart, but that isn't really a big deal for this app:
    rocket_config.secret_key = rocket::config::SecretKey::generate()
        .ok_or_else(|| anyhow::anyhow!("Failed to generate a secret key: not enough system randomness"))?;

    let mut rocket = rocket::custom(rocket_config)
        .manage(state::State {
            // Ensure that we don't need anything more than what
            // the `store_interface::Store` trait provides by
            // only providing that.
            store: Box::new(store),
            static_files: opts.static_files.clone()
        })
        .mount("/api", api::routes());

    // Serve external static files if asked to (useful for dev), else serve embedded files:
    if let Some(path) = opts.static_files {
        rocket = rocket.mount("/", rocket_contrib::serve::StaticFiles::from(path));
    } else {
        rocket = rocket.mount("/", static_files::static_files_route());
    }

    rocket.launch().await?;

    Ok(())
}

/// Prompt for password-like input (input is hidden)
async fn prompt_for_hidden_input(msg: &str) -> anyhow::Result<String> {
    let msg = msg.to_owned();
    tokio::task::spawn_blocking(move ||
        rpassword::prompt_password_stderr(&msg)
            .with_context(|| format!("Failed to read password from stdin"))
    ).await?
}
