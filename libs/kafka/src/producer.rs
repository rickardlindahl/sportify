use std::time::{Duration, SystemTime, UNIX_EPOCH};

use rdkafka::config::ClientConfig;
use rdkafka::error::KafkaError;
use rdkafka::message::OwnedMessage;
use rdkafka::producer::{FutureProducer, FutureRecord};

#[derive(Clone)]
pub struct KafkaProducer {
    producer: FutureProducer,
}

impl KafkaProducer {
    /// Create a new KafkaProducer instance by creating a FutureProducer
    ///
    /// # Examples
    /// Basic usage:
    ///
    /// ```rust norun
    /// let kproducer = KafkaProducer::new("localhost:9092", "username", "password");
    /// ```
    pub fn new(brokers: &str, username: &str, password: &str) -> KafkaProducer {
        // Create the `FutureProducer` to produce asynchronously.
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", brokers)
            .set("security.protocol", "SASL_SSL")
            .set("sasl.mechanisms", "SCRAM-SHA-256")
            .set("sasl.username", username)
            .set("sasl.password", password)
            .create()
            .expect("Producer creation error");

        KafkaProducer { producer }
    }

    fn get_timestamp_miliseconds(now: SystemTime) -> i64 {
        let duration_since_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
        (duration_since_epoch.as_secs() as i64) * 1000
            + i64::from(duration_since_epoch.subsec_millis())
    }

    /// Publish a record to a given topic on Kafka.
    /// pub async fn produce(&self, topic: &str, key: &str, data: &[u8]) ->
    pub async fn produce(
        &self,
        topic: &str,
        key: &str,
        data: &[u8],
    ) -> Result<(i32, i64), (KafkaError, OwnedMessage)> {
        let record = FutureRecord::to(topic)
            .key(key)
            .payload(data)
            .timestamp(Self::get_timestamp_miliseconds(SystemTime::now()));

        self.producer.send(record, Duration::from_secs(0)).await
    }
}
