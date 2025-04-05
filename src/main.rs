use data_collection::hypersync::{client::HyperSyncClient};

mod data_collection;
mod processing;
mod query;
mod storage;


#[tokio::main]
async fn main() {
    println!("Hello World");
}