// ANCHOR: here
use std::thread;

pub struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
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

        let mut threads = Vec::with_capacity(size);

        for _ in 0..size {
            // cria algumas threads e as armazena no vetor
        }

        ThreadPool { threads }
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
// ANCHOR_END: here
