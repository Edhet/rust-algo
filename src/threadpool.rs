use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        mpsc::{self, Sender},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
    time::Duration,
};

pub struct Threadpool {
    threads: Vec<JoinHandle<()>>,
    task_sender: Mutex<Sender<Arc<dyn Fn() -> () + Send + Sync>>>,
    running_tasks: Arc<AtomicUsize>,
}

impl Threadpool {
    pub fn new(thread_amount: i32) -> Self {
        let mut threads = vec![];
        let running_tasks = Arc::new(AtomicUsize::new(0));

        let channel_tuple = mpsc::channel::<Arc<dyn Fn() -> () + Send + Sync>>();
        let (task_sender, task_receiver) = (
            Mutex::new(channel_tuple.0),
            Arc::new(Mutex::new(channel_tuple.1)),
        );

        for _0 in 0..thread_amount {
            let copy_arc_receiver = task_receiver.clone();
            let copy_counter = running_tasks.clone();

            let thread = thread::Builder::new()
                .spawn(move || loop {
                    let msg_receiver = copy_arc_receiver.lock().unwrap();
                    let message = msg_receiver.recv();
                    match message {
                        Ok(task) => task(),
                        Err(..) => break,
                    };
                    copy_counter.fetch_sub(1, Ordering::SeqCst);
                })
                .unwrap();
            threads.push(thread);
        }

        Self {
            threads,
            task_sender,
            running_tasks,
        }
    }

    pub fn execute(&self, function: Arc<dyn Fn() -> () + Send + Sync>) -> () {
        self.running_tasks.fetch_add(1, Ordering::SeqCst);
        let _ = self.task_sender.lock().unwrap().send(function);
    }

    pub fn join(&self) {
        while self
            .running_tasks
            .compare_exchange(0, 0, Ordering::SeqCst, Ordering::SeqCst)
            .is_err()
        {
            thread::sleep(Duration::from_nanos(1000));
        }
    }

    pub fn finish(self) {
        self.join();
    }
}
