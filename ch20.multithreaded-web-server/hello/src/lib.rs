use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            // loop because we want the worker to continue processing a job when it gets one
            // while let (if let, and match) does not drop temporary values until the end of the associated block
            // whereas let does, so the lock can be released as all temporary variables on the right-hand-side immediately goes out of scope
            // when the statement ends.
            let job = receiver.lock().unwrap().recv().unwrap();

            println!("Worker {} got a job; executing.", id);

            job();
        });

        Worker { id, thread }
    }
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    ///
    // usize is chosen because negative size wouldn't make sense
    // we need to put a limit on the thread pool to prevent DoS
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);

        let (sender, receiver) = mpsc::channel();

        // we want all workers to share the own the same receiver
        // Arc is used to allow multiple owners in a thread-safe way
        // receiver is wrapped in a Mutex so that one worker gets to use the receiver at a time
        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)))
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}
