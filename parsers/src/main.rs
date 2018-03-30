mod layout;
mod foo;

fn main() {
    let ugr = String::from("170004");
    let ptres = "089280".to_string();
    let record = layout::Record{ugr: ugr, ptres: ptres};

    println!("Register: <ugr: {}, ptres: {}>", record.ugr, record.ptres);
}
