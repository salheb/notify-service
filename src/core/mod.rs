use log::info;
use simple_logger::SimpleLogger;

use crate::adapters::kafka_inbound::kafka_inbound::start_kafka_consumers;
use crate::core::util::load_env;

pub mod util;

pub async fn start_service(){
    SimpleLogger::new().with_colors(true).init().unwrap();
    info!("Starting service...");

    load_env();

    start_kafka_consumers().await
}