use std::fmt;
use std::{
    error::Error,
    sync::{mpsc, Arc, Mutex},
    thread,
};

// custom PoolCreationError
#[derive(Debug)]
pub struct PoolCreationError {
    message: String,
}

impl PoolCreationError {
    fn new(msg: &str) -> PoolCreationError {
        PoolCreationError {
            message: String::from(msg),
        }
    }
}
impl fmt::Display for PoolCreationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PoolCreationError {}", self.message)
    }
}
impl Error for PoolCreationError {}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool. Must be greater than 0.
    ///
    /// # Returns
    ///
    /// ThreadPool if successful, PoolCreationError otherwise
    /// (size entered is less than 0).
    ///
    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size > 0 {
            let (sender, receiver) = mpsc::channel();
            let receiver = Arc::new(Mutex::new(receiver));

            let mut workers = Vec::with_capacity(size);

            for id in 0..size {
                let worker = Worker::build(id, Arc::clone(&receiver)).unwrap();
                workers.push(worker);
            }

            Ok(ThreadPool { workers, sender })
        } else {
            Err(PoolCreationError::new("Size must be larger than 0."))
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn build(
        id: usize,
        receiver: Arc<Mutex<mpsc::Receiver<Job>>>,
    ) -> Result<Worker, PoolCreationError> {
        let builder = thread::Builder::new();
        let thread = match builder.spawn(move || loop {
            // call to recv will block until a task becomes available.
            // Must use let here (as opposed to if let or while let) so that the
            // temporary mutex variable is dropped (thus releasing the lock),
            // allowing for our servor to process requests in parallel.
            let job = receiver
                .lock()
                .expect("Poisoned mutex. Another thread may have panicked.")
                .recv()
                .unwrap();

            println!("Worker {id} got a job; executing.");
            job();
        }) {
            Ok(thread) => thread,
            Err(_) => {
                return Err(PoolCreationError::new(
                    "Could not spawn the thread of id: {id}.",
                ))
            }
        };
        Ok(Worker { id, thread })
    }
}
