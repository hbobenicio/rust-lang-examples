async fn foo(i: i32) {
    println!("{:02}. Hello, Async syntax!", i);
}

fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();

    for i in 1..=10 {
        rt.spawn(foo(i));
    }

    rt.shutdown_on_idle();
}
