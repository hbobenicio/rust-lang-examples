use rand::prelude::*;
use prometheus::Encoder;

fn main() {
    let parity_counter: prometheus::IntCounterVec = {
        let name = "parity_counters";
        let help = "parity counters: one for even numbers and the other for odd numbers";
        let options = prometheus::Opts::new(name, help);
        
        prometheus::IntCounterVec::new(options, &["parity"]).unwrap()
    };
    let total_random_generated_counter: prometheus::IntCounter = {
        let name = "total_random_generated_counter";
        let help = "counts how much random numbers were generated";
        prometheus::IntCounter::new(name, help).unwrap()
    };
    
    let registry = prometheus::Registry::new();
    registry.register(Box::new(parity_counter.clone())).unwrap();
    registry.register(Box::new(total_random_generated_counter.clone())).unwrap();

    let mut rng = rand::thread_rng();
    for _i in 0..10 {
        let num: i64 = rng.gen_range(1, 7); // num E [x, y)
        total_random_generated_counter.inc();

        if num % 2 == 0 {
            parity_counter.with_label_values(&["even"]).inc();
        } else {
            parity_counter.with_label_values(&["odd"]).inc();
        }
    }

    let metrics = registry.gather();
    println!("{}", metrics_serialize_to_text(&metrics));
}


fn metrics_serialize_to_text(metrics: &[prometheus::proto::MetricFamily]) -> String {
    let mut buffer = Vec::new();
    let encoder = prometheus::TextEncoder::new();
    encoder.encode(&metrics, &mut buffer).unwrap();

    String::from_utf8(buffer).unwrap()
}
