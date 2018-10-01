use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use rand::Rng;

#[derive(Debug, Copy, Clone)]
pub struct ProductionMessage {
    pub miner_id: u8,
    pub ore_quantity: u32
}

pub trait Miner {
    fn work(&self) -> u32 {
      let work: u32 = rand::thread_rng().gen_range(1, 5);
      thread::sleep(Duration::from_secs(work as u64));
      work
    }
}

#[derive(Debug)]
pub struct StandardMiner {
    id: u8
}

impl StandardMiner {
    pub fn new(id: u8) -> StandardMiner {
      StandardMiner { id }
    }
}

impl Miner for StandardMiner {

}


pub fn start(miner_quantity: u8) -> Result<(), String> {
  let mut threads = vec![];

  let (tx, rx) = mpsc::channel();

  for i in 0..miner_quantity {

    let txm = mpsc::Sender::clone(&tx);
    let handle = thread::spawn(move || {

      let miner_id = i + 1;
      let miner = StandardMiner::new(miner_id);
      let ore_quantity = miner.work();

      txm.send(ProductionMessage{ miner_id, ore_quantity }).unwrap();
    });

    threads.push(handle);
  }

  for msg in rx {
    println!("[MAIN] Miner {} produced {} ores", msg.miner_id, msg.ore_quantity);
  }

  Ok(())
}
