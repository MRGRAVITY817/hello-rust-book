// mp: multiple producer, sc: single consumer
// We send from lots of different stream, but received point is only one!
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn basic_tx_rx() {
  // tx: transmission
  // rx: receiver
  let (tx, rx) = mpsc::channel();

  // Move main thread's tx to spawned thread
  thread::spawn(move || {
    let val = String::from("hi");
    tx.send(val).unwrap();
    // println!("val is {}", val); <- tx.send() takes the ownership
    // (when sent, it's gone man.)
  });

  // We can receive the transmitted value from main thread
  let received = rx.recv().unwrap();
  println!("Got: {}", received);
}

pub fn multiple_tx() {
  let (tx, rx) = mpsc::channel();

  thread::spawn(move || {
    let vals = vec![
      String::from("hi"),
      String::from("from"),
      String::from("the"),
      String::from("thread"),
    ];

    for val in vals {
      tx.send(val).unwrap();
      // Give interval of 1second between each thread
      thread::sleep(Duration::from_secs(1));
    }
  });

  for received in rx {
    println!("Got: {}", received);
  }
}

pub fn multiple_producers() {
  let (tx, rx) = mpsc::channel();

  // Cloning tx
  let tx1 = tx.clone();

  // original tx
  thread::spawn(move || {
    let vals = vec![
      String::from("hi"),
      String::from("from"),
      String::from("the"),
      String::from("thread"),
    ];

    for val in vals {
      tx.send(val).unwrap();
      // Give interval of 1second between each thread
      thread::sleep(Duration::from_secs(1));
    }
  });
  // clone tx1
  thread::spawn(move || {
    let vals = vec![
      String::from("bye"),
      String::from("to"),
      String::from("an"),
      String::from("apple"),
    ];

    for val in vals {
      tx1.send(val).unwrap();
      // Give interval of 1second between each thread
      thread::sleep(Duration::from_secs(1));
    }
  });

  for received in rx {
    println!("Got: {}", received);
  }
}
