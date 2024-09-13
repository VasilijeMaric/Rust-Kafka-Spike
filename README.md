# Rust-Kafka-Spike

This is a spike project to test a simple Kafka streaming data

# Prerequisite
- Docker
- CMake

# Get Started
## Install and run a kafka broker
```
cd kafka-broker && docker compose up -d
cd ../
```
## Run the producer 
```
cargo run -- produce
```
## Run the consumer
```
cargo run -- consume
```

You should see the following after several seconds
```
Received message: Hello, Kafka!
```
