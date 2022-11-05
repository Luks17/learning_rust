
use std::{thread, sync::{mpsc, Arc, Mutex}};


type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
  NewJob(Job),
  Terminate,
}

pub struct ThreadPool {
  workers: Vec<Worker>,
  sender: mpsc::Sender<Message>,
}

impl ThreadPool {
/// Create a new ThreadPool.
/// 
/// # Parameters
/// 
/// The size parameter is the number of threads idling in the new pool.
/// 
/// # Functionality
/// 
/// The new ThreadPool automatically creates a mpsc sender to send jobs (private type alias that holds closures) to the threads.
/// Each thread is created inside a Worker (private struct) that has it's own unique ID.
/// Each worker gets shared ownership of a mutable mpsc receiver using Arc and Mutex Smart Pointers to get those features.
///
/// 
/// # Panics
/// 
/// The `new` function will panic if the size is zero
  pub fn new(size: usize) -> ThreadPool {
    assert!(size>0);

    let (sender, receiver) = mpsc::channel();
    let receiver = Arc::new(Mutex::new(receiver));

    let mut workers = Vec::with_capacity(size);

    for id in 0..size {
      workers.push(Worker::new(id, Arc::clone(&receiver)));
    }

    ThreadPool { workers, sender }
  }

  pub fn execute<F>(self: &Self, f: F)
  where F: FnOnce() + Send + 'static {
    let job = Box::new(f);

    self.sender.send(Message::NewJob(job)).unwrap();
  }
}

impl Drop for ThreadPool {
  fn drop(self: &mut Self) {

    println!("Shutting down all workers!");

    // sends terminate message to all workers
    for _ in &self.workers {
      self.sender.send(Message::Terminate).unwrap();
    }

    // makes main thread wait for workers to terminate
    for worker in &mut self.workers {
      println!("Shutting down worker {}", worker.id);

      // take() returns the value of the option and leaves a None where it used to be
      if let Some(thread) = worker.thread.take() {
        thread.join().unwrap();
      }
    }
  }
}

struct Worker {
  id: usize,
  thread: Option<thread::JoinHandle<()>>
}

impl Worker {
  fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
    println!("Worker {id} created!");
    let builder = thread::Builder::new().name(format!("{id}").into());
    let thread = builder.spawn(move || loop {
      let message = receiver.lock().expect("Sender thread panicked sending the job")
                                  .recv().unwrap();

      match message {
        Message::NewJob(job) => {
          println!("Worker {id} got a job.");
          job();
        },
        Message::Terminate => {
          println!("Worker {id} was told to terminate.");
          break;
        },
      }
    }).unwrap();

    Worker { id, thread: Some(thread) }
  }
}

