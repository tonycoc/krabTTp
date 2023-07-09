use std::sync::{mpsc, Arc, Mutex};
use std::thread;
pub struct ThreadPool {

    workers:Vec<Worker>,

    sender: mpsc::Sender<Job>
}


type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct Worker {

    id: usize,

    thread: thread::JoinHandle<()>
}

impl Worker {

    pub fn new(id:usize, receiver:Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();

            println!("worker {} got a job",id);

            job();
        });


        Worker{id ,thread}
    }

}

impl ThreadPool {

    pub fn new(size:usize) -> ThreadPool {


        let mut workers = Vec::with_capacity(size);

        let (sx,rx) = mpsc::channel();

        let res = Arc::new(Mutex::new(rx));


        for id in 0..size {
            workers.push(
                Worker::new(id,Arc::clone(&res))
            )
        }



        ThreadPool{workers, sender:sx}

    }


    pub fn execute<F>(&self,func:F)
    where
        F:FnOnce() + Send + 'static,
        {
            let job = Box::new(func);

            self.sender.send(job).unwrap();
        }
}
