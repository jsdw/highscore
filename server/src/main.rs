mod store;
mod sessions;

use structopt::StructOpt;
use std::path::PathBuf;
use warp::Filter;

#[derive(Debug,Clone,StructOpt)]
struct Opts {
    /// Which port will the app run on
    #[structopt(long,short)]
    port: u16,
    /// Where do the client files live
    #[structopt(long,short,parse(from_os_str))]
    static_files: PathBuf
}

#[tokio::main]
async fn main() {

    let opts = Opts::from_args();
    println!("{:?}", opts);

    let get_summary = warp::path("get_summary");

    let static_files = warp::fs::dir(opts.static_files);

    let routes = static_files;

    warp::serve(routes).run(([127, 0, 0, 1], opts.port)).await;

}