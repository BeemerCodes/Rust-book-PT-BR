use std::{sync::mpsc, thread};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

struct Job;

// ANCHOR: here
impl ThreadPool {
    // --snip--
    // ANCHOR_END: here
    /// Cria um novo ThreadPool.
    ///
    /// O tamanho (`size`) é o número de threads no pool.
    ///
    /// # Pânicos
    ///
    /// A função `new` entrará em pânico se o tamanho for zero.
    // ANCHOR: here
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, receiver));
        }

        ThreadPool { workers, sender }
    }
    // --snip--
    // ANCHOR_END: here

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
    // ANCHOR: here
}

// --snip--

// ANCHOR_END: here

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

// ANCHOR: here
impl Worker {
    fn new(id: usize, receiver: mpsc::Receiver<Job>) -> Worker {
        let thread = thread::spawn(|| {
            receiver;
        });

        Worker { id, thread }
    }
}
// ANCHOR_END: here
