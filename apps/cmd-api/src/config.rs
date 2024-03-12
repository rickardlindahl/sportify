use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub kafka_broker: String,
    pub kafka_username: String,
    pub kafka_password: String,
    pub kafka_topic_teams: String,
}

impl Config {
    pub fn new() -> Self {
        let kafka_broker = env::var("KAFKA_BROKER").expect("KAFKA_BROKER not set");
        let kafka_username = env::var("KAFKA_USERNAME").expect("KAFKA_USERNAME not set");
        let kafka_password = env::var("KAFKA_PASSWORD").expect("KAFKA_PASSWORD not set");
        let kafka_topic_teams = env::var("KAFKA_TOPIC_TEAMS").expect("KAFKA_TOPIC_TEAMS not set");

        Config {
            kafka_broker,
            kafka_username,
            kafka_password,
            kafka_topic_teams,
        }
    }
}
