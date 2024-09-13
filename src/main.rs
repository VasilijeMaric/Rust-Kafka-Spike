use rdkafka::config::ClientConfig;
use rdkafka::Message;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::consumer::{StreamConsumer, Consumer, CommitMode};
use tokio;
use std::env;
use std::time::Duration;

// Function to produce messages
async fn produce_message(brokers: &str, topic: &str, key: &str, payload: &str) {
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .create()
        .expect("Producer creation error");

    println!("Got the producer");

    let delivery_status = producer
        .send(
            FutureRecord::to(topic)
                .key(key)
                .payload(payload),
            Duration::from_secs(0),
        )
        .await;

    println!("Confirming delivery status.... ");

    match delivery_status {
        Ok(delivery) => println!("Sent: {:?}", delivery),
        Err((e, _)) => eprintln!("Error sending message: {:?}", e),
    }
}

// Function to consume messages
async fn consume_message(brokers: &str, group_id: &str, topics: &[&str]) {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", group_id)
        .set("bootstrap.servers", brokers)
        .set("enable.auto.commit", "true")
        .create()
        .expect("Consumer creation error");

    consumer
        .subscribe(topics)
        .expect("Can't subscribe to specified topics");

    println!("Listening for messages...");
    loop {
        match consumer.recv().await {
            Ok(message) => {
                if let Some(payload) = message.payload() {
                    println!("Received message: {}", String::from_utf8_lossy(payload));
                }
                consumer.commit_message(&message, CommitMode::Async).unwrap();
            }
            Err(e) => {
                eprintln!("Error receiving message: {:?}", e);
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    // Check if "produce" or "consume" argument is passed
    if args.len() < 2 {
        eprintln!("Usage: cargo run -- [produce|consume]");
        return;
    }

    let brokers = "localhost:29092";
    let topic = "my-topic";

    if args[1] == "produce" {
        let key = "my-key";
        let payload = "Hello, Kafka!";
        produce_message(brokers, topic, key, payload).await;

    } else if args[1] == "consume" {
        let group_id = "my-consumer-group";
        let topics = [topic];
        consume_message(brokers, group_id, &topics).await;

    } else {
        eprintln!("Unknown command. Use 'produce' or 'consume'.");
    }
}
