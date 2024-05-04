use rdkafka::{producer::FutureProducer, ClientConfig};

fn main() {
    let brokers = "localhost:9092";
    let producer: &FutureProducer = &ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error");
}

#[derive(Debug)]
struct Order {
    pub order: String,
}
