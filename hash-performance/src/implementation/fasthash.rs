use std::collections::HashMap;
use crate::constant;

type Hasher = fasthash::RandomState<fasthash::city::Hash64>;

pub fn run(insert_count: usize) {
    flame::start("fasthash::with_capacity");
    with_capacity(insert_count);
    flame::end("fasthash::with_capacity");
}

pub fn with_capacity(insert_count: usize) {
    let mut h: HashMap<String, String, Hasher> = 
        HashMap::with_capacity_and_hasher(insert_count, Hasher::new());

    for i in 1..=insert_count {
        let key = format!("{}{}", constant::PREFIX, i);
        let value = key.clone();
        h.insert(key, value);
    }
}

