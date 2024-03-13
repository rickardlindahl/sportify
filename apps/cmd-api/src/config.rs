use sportify_kafka::KafkaProducer;
use std::env;

#[derive(Clone)]
pub struct Config {
    pub kafka_producer: KafkaProducer,
    pub kafka_topic_teams: String,
}

impl Config {
    pub fn new() -> Self {
        let kafka_brokers = env::var("KAFKA_BROKERS").expect("KAFKA_BROKERS not set");
        let kafka_username = env::var("KAFKA_USERNAME").expect("KAFKA_USERNAME not set");
        let kafka_password = env::var("KAFKA_PASSWORD").expect("KAFKA_PASSWORD not set");
        let kafka_topic_teams = env::var("KAFKA_TOPIC_TEAMS").expect("KAFKA_TOPIC_TEAMS not set");

        let kafka_producer = KafkaProducer::new(&kafka_brokers, &kafka_username, &kafka_password);

        Config {
            kafka_producer,
            kafka_topic_teams,
        }
    }
}
