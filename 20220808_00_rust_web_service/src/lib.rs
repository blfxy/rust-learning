use std::sync::{mpsc, Arc, Mutex};
use std::thread;
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}
type Job = Box<dyn FnOnce() + Send + 'static>;
impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        /// 创建线程池。
        ///
        /// 线程池中线程的数量。
        ///
        /// # Panics
        ///
        /// `new` 函数在 size 为 0 时会 panic。
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        // with_capacity 创建预先分配空间的vec 因为这里知道了vec的大小为size
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool { workers, sender }
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
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("Worker {} got a job; executing.", id);
            job();
        });
        Worker { id, thread }
    }
}
