#[macro_use(Deserialize)]
extern crate serde;

#[macro_use(Error)]
extern crate thiserror;

#[macro_use(lazy_static)]
extern crate lazy_static;

#[macro_use(Parser)]
extern crate clap;

mod cli;
mod config;
mod error;
mod models;
mod parceli;

use clap::Parser;
use cli::{Args, CliArgs};
use config::Config;
use console::style;
use parceli::Parceli;

lazy_static! {
    pub static ref ARGS: Args = Args::from(CliArgs::parse());
    pub static ref CONFIG: Config = Config::load();
}

// Lightweight program should only need about 2 threads
#[tokio::main(worker_threads = 2)]
async fn main() {
    let parceli = Parceli::new();
    let results = parceli.track().await;

    if let Err(e) = &results {
        eprintln!("[{}] - An Error Has Occured - {}", style("ERROR").red(), e);
    }

    let parcels = results.unwrap();

    for parcel in parcels {
        println!("{}", parcel);
    }
}
