extern crate libc;

#[link(name = "calc", kind = "static")]
extern {
    fn add(x: libc::c_int, y: libc::c_int) -> libc::c_int;
}

fn main() {
    let result = unsafe {
        add(2, 3)
    };

    println!("Result: {}", result);
}
