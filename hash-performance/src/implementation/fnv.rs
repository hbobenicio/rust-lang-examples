use fnv::FnvHashMap;
use crate::constant;

pub fn run(insert_count: usize) {
    flame::start("fnv::empty");
    empty(insert_count);
    flame::end("fnv::empty");

    flame::start("fnv::with_capacity");
    with_capacity(insert_count);
    flame::end("fnv::with_capacity");
}

pub fn empty(insert_count: usize) {
    let mut h: FnvHashMap<String, String> = FnvHashMap::default();
    
    for i in 1..=insert_count {
        let key = format!("{}{}", constant::PREFIX, i);
        let value = key.clone();
        h.insert(key, value);
    }
}

pub fn with_capacity(insert_count: usize) {
    let mut h: FnvHashMap<String, String> =
        FnvHashMap::with_capacity_and_hasher(insert_count, Default::default());
    
    for i in 1..=insert_count {
        let key = format!("{}{}", constant::PREFIX, i);
        let value = key.clone();
        h.insert(key, value);
    }
}
