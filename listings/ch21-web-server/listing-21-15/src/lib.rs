// ANCHOR: here
use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
}

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

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id));
        }

        ThreadPool { workers }
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

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize) -> Worker {
        let thread = thread::spawn(|| {});

        Worker { id, thread }
    }
}
// ANCHOR_END: here
