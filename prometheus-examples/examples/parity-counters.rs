use rand::prelude::*;
use prometheus::Encoder;

fn main() {
    let parity_counters: prometheus::IntCounterVec = {
        let name = "parity_counters";
        let help = "parity counters: one for even numbers and the other for odd numbers";
        let options = prometheus::Opts::new(name, help)
            .namespace("hbo")
            .const_label("app", "prometheus-examples");
        
        prometheus::IntCounterVec::new(options, &["parity"]).unwrap()
    };
    let total_random_generated_counter: prometheus::IntCounter = {
        let name = "total_random_generated_counter";
        let help = "counts how much random numbers were generated";
        let options = prometheus::Opts::new(name, help)
            .namespace("hbo")
            .const_label("app", "prometheus-examples");

        prometheus::IntCounter::with_opts(options).unwrap()
    };
    let parity_distribution_gauges: prometheus::GaugeVec = {
        let name = "parity_distribution_gauges";
        let help = "measures (in %) how much evens and odds were randomized";
        let options = prometheus::Opts::new(name, help)
            .namespace("hbo")
            .const_label("app", "prometheus-examples");
        prometheus::GaugeVec::new(options, &["parity"]).unwrap()
    };
    let num_histogram: prometheus::Histogram = {
        let name = "num_histogram";
        let help = "number histogram (to count repetitions)";
        let options = prometheus::HistogramOpts::new(name, help)
            .namespace("hbo")
            .const_label("app", "prometheus-examples")
            .buckets((1..10).map(|x| x as f64).collect());
        prometheus::Histogram::with_opts(options).unwrap()
    };
    
    let registry = prometheus::Registry::new();
    registry.register(Box::new(parity_counters.clone())).unwrap();
    registry.register(Box::new(total_random_generated_counter.clone())).unwrap();
    registry.register(Box::new(parity_distribution_gauges.clone())).unwrap();
    registry.register(Box::new(num_histogram.clone())).unwrap();

    let mut rng = rand::thread_rng();
    for _ in 0..100 {
        let num: i64 = rng.gen_range(1, 10 + 1); // num E [x, y)
        total_random_generated_counter.inc();
        num_histogram.observe(num as f64);

        if num % 2 == 0 {
            parity_counters.with_label_values(&["even"]).inc();
            let even_count: i64 = parity_counters.get_metric_with_label_values(
                &["even"]
            ).unwrap().get();

            let percentage: f64 = (even_count as f64) / (total_random_generated_counter.get() as f64);
            parity_distribution_gauges.with_label_values(&["even"]).set(percentage);
        } else {
            parity_counters.with_label_values(&["odd"]).inc();
            let odd_count: i64 = parity_counters.get_metric_with_label_values(
                &["odd"]
            ).unwrap().get();

            let percentage: f64 = (odd_count as f64) / (total_random_generated_counter.get() as f64);
            parity_distribution_gauges.with_label_values(&["odd"]).set(percentage);
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
