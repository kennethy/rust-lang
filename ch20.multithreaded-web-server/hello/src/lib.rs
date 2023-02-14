use std::{sync::{mpsc, Arc, Mutex}, thread};

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let mut workers = Vec::with_capacity(size);
        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..size {
            // Arc allows the receiver to have multiple owners
            // Mutex ensures only one worker gets a job from the receiver at a time
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender: Some(sender) }
    }

    // () after FnOnce represents a closure that takes no params
    // and returns the unit type ()
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {

            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");
                    job();
                }
                Err(_) => {
                    println!("Worker {id} disconnected, shutting it down.");
                    break;
                }
            }
        });


        Worker { id, thread: Some(thread) }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.workers {

            // dropping a sender closes the channel
            drop(self.sender.take());

            println!("shutting down worker {}", worker.id);

            // take() on Option takes the Some variant out and leaves None in its place
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
