use std::collections::HashMap;

const PATH: &str = "PATH";

fn main() {
    if let Err(err) = run() {
        eprintln!("error: {}", err);
    }
}

fn run() -> Result<(), String> {
    let path_env_var: String = std::env::var(PATH).map_err(map_err_to_string)?;

    let mut histogram: HashMap<&str, usize> = HashMap::new();

    for var in path_env_var.split(":") {
        let mut count: usize = *histogram.get(var).unwrap_or(&0);
        count += 1;
        histogram.insert(var, count);
    }

    // checking for duplicate entries
    let mut found_duplicate = false;
    for (var, count) in &histogram {
        if *count > 1 {
            found_duplicate = true;
            eprintln!("entry \"{}\" has {} occurrencies!", var, count);
        }
    }
    if found_duplicate {
        return Err(String::from("found duplicates"));
    }
    
    println!("PATH is ok.");
    Ok(())
}

fn map_err_to_string<E: std::error::Error>(err: E) -> String {
    err.to_string()
}
