use rdkafka::client::ClientContext;
use rdkafka::config::ClientConfig;
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::consumer::{CommitMode, Consumer, ConsumerContext, Rebalance};
use rdkafka::error::KafkaResult;
use rdkafka::message::{Headers, Message};
use rdkafka::topic_partition_list::TopicPartitionList;
use log::{info, warn};

use crate::core::util;

// A context can be used to change the behavior of producers and consumers by adding callbacks
// that will be executed by librdkafka.
// This particular context sets up custom callbacks to log rebalancing events.
struct CustomContext;

impl ClientContext for CustomContext {}

impl ConsumerContext for CustomContext {
    fn pre_rebalance(&self, rebalance: &Rebalance) {
        info!("Pre rebalance {:?}", rebalance);
    }

    fn post_rebalance(&self, rebalance: &Rebalance) {
        info!("Post rebalance {:?}", rebalance);
    }

    fn commit_callback(&self, result: KafkaResult<()>, _offsets: &TopicPartitionList) {
        info!("Committing offsets: {:?}", result);
    }
}

// A type alias with your custom consumer can be created for convenience.
//type LoggingConsumer = StreamConsumer<CustomContext>;

pub async fn start_kafka_consumers(){
    //let context = CustomContext;
    log::info!("Creating consumer within server {} and group id {}", util::get_env_value("kakfa_broker"), util::get_env_value("group_id"));
    let consumer: StreamConsumer = ClientConfig::new()
        .set("bootstrap.servers", util::get_env_value("kakfa_broker"))
        .set("group.id", util::get_env_value("group_id"))
        .set("enable.partition.eof", "false")
        //.set("security.protocol", util::get_env_value("security_protocol"))
        //.set("sasl.mechanisms", util::get_env_value("sasl_mechanisms"))
        //.set("sasl.username", util::get_env_value("sasl_username"))
        //.set("sasl.password", util::get_env_value("sasl_password"))
        .set("session.timeout.ms", util::get_env_value("session_timeout_ms"))
        .set("enable.auto.commit", "false")
        
        .create()
        .expect("Failed to connect to kafka server");

    let topic = util::get_env_value("topic_messaging");
    //let topic_dlq = util::get_env_value("topic_messaging_dlq");
    log::info!("Topic name {}", topic);

    consumer
        .subscribe(&[&topic])
        .expect("Can't subscribe to specified topic: &topic.");


    loop {
        match consumer.recv().await {
            Err(e) => warn!("Kafka error: {}", e),
            Ok(m) => {
                let payload = match m.payload_view::<str>() {
                    None => "",
                    Some(Ok(s)) => s,
                    Some(Err(e)) => {
                        warn!("Error while deserializing message payload: {:?}", e);
                        ""
                    }
                };
                log::info!("key: '{:?}', payload: '{}', topic: {}, partition: {}, offset: {}, timestamp: {:?}",
                        m.key(), payload, m.topic(), m.partition(), m.offset(), m.timestamp());
                if let Some(headers) = m.headers() {
                    for header in headers.iter() {
                        info!("  Header {:#?}: {:?}", header.key, header.value);
                    }
                }
                consumer.commit_message(&m, CommitMode::Async).unwrap();
            }
        };
    }
    
}