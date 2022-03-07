use std::{sync::{Arc, Mutex, mpsc}, thread::{self, JoinHandle}, time::Duration};

fn main() {
    let pool: ThreadPool = ThreadPool::new(4);

    for index in 0..9 {
        pool.exec(move || {
            println!("Hello, world! index:{}", index);
        });
    }
    thread::sleep(Duration::from_secs(5));
}

struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    fn new(size: usize) -> ThreadPool {
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    fn exec<F>(&self, f:F)
        where
            F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

type Job = Box<dyn FnBox + Send + 'static>;

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<Self>) {
        (*self)()
    }
}

struct Worker {
    id: usize,
    thread: JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                println!("worker {} got the job", id);
                let job = receiver.lock().unwrap().recv().unwrap();
                job.call_box();
            }
        });
        Worker { id, thread }
    }
}
