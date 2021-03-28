// use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

// Mutex: Mutual Exclusion, which means threads are mutually excluded
// to access data from only one thread at a time.
pub fn basic_mutex() {
  // Mutex<T> is a smart pointer.
  let m = Mutex::new(4);

  {
    // Once lock is acquired, we can get the value 4
    let mut num = m.lock().unwrap();
    // Then we use mutable reference to change value of Mutex
    *num = 6;
  } // When lock() goes out of scope, it drops the lock.

  // Hence, this will print 6.
  println!("m = {:?}", m);
}

pub fn multithread_mutex() {
  // let counter = Rc::new(Mutex::new(0));
  // Since we cannot use Rc with threading, we use special Rc,
  // called Atomic RC, Arc.
  let counter = Arc::new(Mutex::new(0));
  let mut handles = vec![];

  for _ in 0..10 {
    // let counter = Rc::clone(&counter);
    let counter = Arc::clone(&counter);
    let handle = thread::spawn(move || {
      let mut num = counter.lock().unwrap();
      *num += 1;
    });
    handles.push(handle);
  }

  for handle in handles {
    // wait for all threads
    handle.join().unwrap();
  }

  // Similar to RefCell, though counter was declared immutable
  // but can be referred as mutable, thanks to Mutex.
  println!("Result: {}", *counter.lock().unwrap());
}
