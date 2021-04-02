use std::sync::{mpsc, Arc, Mutex};
use std::thread;

enum Message {
  NewJob(Job),
  Terminate,
}

/// ThreadPool is a group of spawned threads that are waiting to handle a task
pub struct ThreadPool {
  workers: Vec<Worker>,
  sender: mpsc::Sender<Message>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
  /// Create a new ThreadPool.
  ///
  /// The size is the number of threads in the pool.
  ///
  /// # Panics
  /// The `new` function will panic if the size is zero.
  pub fn new(size: usize) -> ThreadPool {
    assert!(size > 0);

    let (sender, receiver) = mpsc::channel();
    let receiver = Arc::new(Mutex::new(receiver));
    let mut workers = Vec::with_capacity(size);
    for id in 0..size {
      workers.push(Worker::new(id, Arc::clone(&receiver)));
    }
    ThreadPool { workers, sender }
  }
  /// Execute workers with a new job.
  pub fn execute<F>(&self, f: F)
  where
    F: FnOnce() + Send + 'static,
  {
    let job = Box::new(f);
    self.sender.send(Message::NewJob(job)).unwrap();
  }
}

impl Drop for ThreadPool {
  /// Drop workers.
  ///
  /// This method first takes the thread out of worker using `take()`
  ///
  /// so thread gets dropped after the scope
  fn drop(&mut self) {
    println!("Sending terminate message to all workers.");
    // The reason we separate loops is to avoid deadlocks.
    for _ in &self.workers {
      self.sender.send(Message::Terminate).unwrap();
    }
    println!("Shutting down all workers.");
    for worker in &mut self.workers {
      println!("Shutting down worker {}", worker.id);
      // take() takes the value out of Option<T>
      if let Some(thread) = worker.thread.take() {
        thread.join().unwrap();
      }
    }
  }
}

/// Workers make thread and wait for the code
struct Worker {
  id: usize,
  thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
  /// Create a new worker.
  ///
  /// The `id` is an id for each worker.
  ///
  /// The `receiver` is a receiver endpoint for each worker.
  fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
    let thread = thread::spawn(move || loop {
      let message = receiver.lock().unwrap().recv().unwrap();
      match message {
        Message::NewJob(job) => {
          println!("Worker {} got a job; executing", id);
          job();
        }
        Message::Terminate => {
          println!("Worker {} was told to terminate", id);
          break;
        }
      }
    });
    Worker {
      id,
      thread: Some(thread),
    }
  }
}
