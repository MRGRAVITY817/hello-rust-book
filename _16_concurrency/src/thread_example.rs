use std::thread;
use std::time::Duration;

pub fn new_thread() {
  // "thread::spawn()" makes extra thread
  thread::spawn(|| {
    for i in 1..10 {
      println!("Hi number {} from the spawned thread!", i);
      thread::sleep(Duration::from_millis(1));
    }
  });

  // Normal main thread
  for i in 1..5 {
    println!("Hi number {} from the main thread!", i);
    thread::sleep(Duration::from_millis(1));
  }

  // This program ends when the main thread is over, even extra threads
  // should still be running
}

pub fn wait_thread() {
  let handle = thread::spawn(|| {
    for i in 1..10 {
      println!("Hi number {} from the spawned thread!", i);
      thread::sleep(Duration::from_millis(1));
    }
  });
  for i in 1..5 {
    println!("Hi number {} from the main thread!", i);
    thread::sleep(Duration::from_millis(1));
  }
  // This will wait for extra thread to end
  handle.join().unwrap();
  // If this goes before main for loop, it will first wait extra thread to end,
  // and proceed with main thread
}

pub fn move_closure() {
  // Declared in main thread
  let v = vec![1, 2, 3];

  // "move" keyword moves certain resource to other thread
  let handle = thread::spawn(move || {
    println!("Here's a vector: {:?}", v);
  });

  // drop(v); <- since v has moved to other thread, this is impossible

  handle.join().unwrap();
}
