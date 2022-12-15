use std::{fmt, thread};

#[derive(Debug)]
pub struct PoolCreationError;

impl fmt::Display for PoolCreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "size must be greater than zero")
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize) -> Worker {
        let builder = thread::Builder::new();
        let thread = match builder.spawn(|| {}) {
            Err(e) => {
                eprintln!("failed to create thread");
                Err(e)
            },
            Ok(thread) => Ok(thread),
        };
        // Still need to work out how the thread builder works
        let thread = thread.unwrap();
        Worker{ id, thread }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    /// 
    /// The size is the number of threads in the pool.
    /// 
    /// # Panics
    /// 
    /// The `build` function will panic is the size is zero.
    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id));
        }
        match size {
            0 => Err(PoolCreationError),
            _ => Ok(ThreadPool { workers }),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pool_size_zero() {
        assert!(ThreadPool::build(0).is_err())
    }
}