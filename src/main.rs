#![allow(unused)]

use crate::internal::InternalManager;
mod internal;
mod core;
mod types;
mod models;
mod utils;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    InternalManager::new().await;
}
