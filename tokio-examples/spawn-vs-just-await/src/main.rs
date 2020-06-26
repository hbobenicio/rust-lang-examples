fn main() {
    let mut runtime = tokio::runtime::Builder::new()
        // .basic_scheduler()
        .threaded_scheduler()
        .core_threads(2)
        .enable_time()
        .build()
        .expect("tokio runtime creation failed");

    if let Err(err) = runtime.block_on(main_task()) {
        eprintln!("error: {:?}", err);
        std::process::exit(1);
    }
}

async fn main_task() -> Result<(), ()> {
    // let dummy_task = dummy();
    let _dummy_task_1: tokio::task::JoinHandle<()> = tokio::spawn(dummy());
    let _dummy_task_2: tokio::task::JoinHandle<()> = tokio::spawn(dummy());
    let _dummy_task_3: tokio::task::JoinHandle<()> = tokio::spawn(dummy());

    tokio::time::delay_for(std::time::Duration::from_millis(1000)).await;
    println!("info: main: Hello, World from {:?}", std::thread::current().id());

    // dummy_task.await.unwrap();

    Ok(())
}

async fn dummy() {
    println!("info: dummy task here from {:?}", std::thread::current().id());
}
