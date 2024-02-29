import { Kafka } from "kafkajs";

export const kafka = new Kafka({
  clientId: "core-api",
  brokers: ["localhost:9092"],
});

export const producer = kafka.producer();

export async function setupProducers() {
  await producer.connect();
}
