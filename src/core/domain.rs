use rdkafka::config::ClientConfig;

pub struct KakfaConfiguration{
    pub kafka_config: ClientConfig,
    pub kakfa_broker: String,
    pub security_protocol: String,
    pub sasl_mechanisms: String,
    pub sasl_username: String,
    pub sasl_password: String,
    pub topic_message: String,
    pub topic_message_dlq: String
}