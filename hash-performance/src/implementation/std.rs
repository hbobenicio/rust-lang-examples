use std::collections::HashMap;
use crate::constant;

pub fn run(insert_count: usize) {
    flame::start("std::empty");
    empty(insert_count);
    flame::end("std::empty");

    flame::start("std::with_capacity");
    with_capacity(insert_count);
    flame::end("std::with_capacity");
}

pub fn empty(insert_count: usize) {
    let mut h: HashMap<String, String> = HashMap::new();
    
    for i in 1..=insert_count {
        let key = format!("{}{}", constant::PREFIX, i);
        let value = key.clone();
        h.insert(key, value);
    }
}

pub fn with_capacity(insert_count: usize) {
    let mut h: HashMap<String, String> = HashMap::with_capacity(insert_count);
    
    for i in 1..=insert_count {
        let key = format!("{}{}", constant::PREFIX, i);
        let value = key.clone();
        h.insert(key, value);
    }
}
