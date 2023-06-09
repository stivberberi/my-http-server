use std::error::Error;
use std::fmt;
use std::thread;

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
    threads: Vec<thread::JoinHandle<()>>,
}

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
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size > 0 {
            let mut threads = Vec::with_capacity(size);
            for _ in 0..size {
                todo!("Create some threads");
            }

            Ok(ThreadPool { threads })
        } else {
            Err(PoolCreationError::new("Size must be larger than 0."))
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}
