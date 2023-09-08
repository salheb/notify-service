pub mod core;
pub mod adapters;
use crate::core::start_service;

#[tokio::main]
async fn main() {
    start_service().await
}
