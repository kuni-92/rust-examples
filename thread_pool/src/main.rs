use std::thread::JoinHandle;

fn main() {
    let pool: ThreadPool = ThreadPool::new(4);

    pool.exec(|| {
        println!("Hello, world!");
    });
}

struct ThreadPool {
    threads: Vec<JoinHandle<()>>,
}

impl ThreadPool {
    fn new(size: usize) -> ThreadPool {
        let mut threads = Vec::with_capacity(size);
        ThreadPool {
            threads
        }
    }

    fn exec<F>(&self, f:F)
        where
            F: FnOnce() + Send + 'static
    {
    }
}
