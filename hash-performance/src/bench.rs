use crate::implementation;

pub fn run() -> Result<(), String> {
    let insert_count = 5_000_000;

    flame::start("bench::run");

    flame::start("warmup");
    implementation::std::empty(insert_count);
    flame::end("warmup");

    flame::start("std");
    implementation::std::run(insert_count);
    flame::end("std");

    flame::start("fnv");
    implementation::fnv::run(insert_count);
    flame::end("fnv");

    flame::start("fasthash");
    implementation::fasthash::run(insert_count);
    flame::end("fasthash");

    flame::end("bench::run");
    Ok(())
}

