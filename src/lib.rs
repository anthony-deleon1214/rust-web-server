use std::{
    fmt,
    thread,
    sync::{mpsc, Arc, Mutex},
};

#[derive(Debug)]
pub struct PoolCreationError;

impl fmt::Display for PoolCreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "size must be greater than zero")
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<Arc<Mutex<mpsc::Receiver<Job>>>>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let builder = thread::Builder::new();
        let thread = match builder.spawn(move || loop {
                let job = receiver.lock().expect("Unable to acquire mutex lock").recv().unwrap();

                println!("Worker {id} got a job; executing.");

                job();
        }) {
            Err(e) => {
                eprintln!("failed to create thread");
                Err(e)
            },
            Ok(thread) => Ok(thread),
        };
        // Still need to work out how the thread builder works
        let thread = thread.unwrap();
        Worker{ id, thread: Some(thread) }
    }
}

// Job defined as a type alias for a closure
type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
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

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        match size {
            0 => Err(PoolCreationError),
            _ => Ok(ThreadPool { workers, sender }),
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

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

           if let Some(thread) = worker.thread.take() {
               thread.join().unwrap();
           }
        }
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