use std::hash::Hash;
use std::sync::{Arc, mpsc, Mutex};
use std::thread;

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
  id: usize,
  job: Option<thread::JoinHandle<()>>,
}

enum Message {
  Terminate,
  Start(Job),
}

pub struct ThreadPool {
  workers: Vec<Worker>,
  // Sender will automatically dispatch the job to any available worker.
  sender: mpsc::Sender<Message>,
}

impl Worker {
  fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Self {
    Worker {
      id,
      job: Some(thread::spawn(move || loop {
        // Create a dead loop and receive all the jobs via the message channel.
        // Because we pass jobs on the channel, so we call directly call mutex().
        // Block the thread and wait for any available message. If message is
        // received by any other thread, the current thread will not get it
        // and will keep waiting.
        let message = receiver.lock().unwrap().recv().unwrap();
        match message {
          Message::Start(job) => {
            println!("Thread #{}: executing the job...", id);
            job();
          }

          Message::Terminate => {
            println!("Thread #{}: terminating..", id);
            break;
          }
        }
      })),
    }
  }
}

impl Drop for ThreadPool {
  fn drop(&mut self) {
    // Send the termination signal to each thread.
    for _ in 0..self.workers.len() {
      self.sender.send(Message::Terminate).unwrap();
    }

    for worker in self.workers.iter_mut() {
      println!("Shutting down worker #{}", worker.id);

      if let Some(handle) = worker.job.take() {
        handle.join().unwrap();
      }
    }
  }
}

impl ThreadPool {
  pub fn new(thread_num: usize) -> Result<Self, Box<dyn std::error::Error>> {
    assert!(thread_num > 0);

    // Create the connection stubs.
    let (sender, receiver) = mpsc::channel();
    let rx = Arc::new(Mutex::new(receiver));

    let mut workers = Vec::with_capacity(thread_num);
    for id in 0..thread_num {
      // create some threads and store them in the vector
      workers.push(Worker::new(id, Arc::clone(&rx)));
    }

    Ok(ThreadPool { workers, sender })
  }

  pub fn execute<F>(&self, f: F) -> Result<(), Box<dyn std::error::Error>>
    where F: FnOnce() + Send + 'static {
    let job = Box::new(f);

    if let Err(e) = self.sender.send(Message::Start(job)) {
      let error_message =
        format!("Cannot execute the job due to {}", e.to_string());
      Err(error_message.into())
    } else {
      Ok(())
    }
  }
}