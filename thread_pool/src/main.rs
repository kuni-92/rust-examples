use std::thread::{self, JoinHandle};

fn main() {
    let pool: ThreadPool = ThreadPool::new(4);

    pool.exec(|| {
        println!("Hello, world!");
    });
}

struct ThreadPool {
    workers: Vec<Worker>,
}

impl ThreadPool {
    fn new(size: usize) -> ThreadPool {
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id));
        }

        ThreadPool { workers }
    }

    fn exec<F>(&self, f:F)
        where
            F: FnOnce() + Send + 'static
    {
        f()
    }
}

struct Worker {
    id: usize,
    thread: JoinHandle<()>,
}

impl Worker {
    fn new(id: usize) -> Worker {
        let thread = thread::spawn(|| {});
        Worker { id, thread }
    }
}
