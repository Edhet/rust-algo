use std::{
    ops::DerefMut,
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex, RwLock,
    },
    thread::{self, JoinHandle},
    time::Duration,
};

pub struct Threadpool {
    threads: Vec<JoinHandle<()>>,
    task_sender: Mutex<Sender<Arc<dyn Fn() -> () + Send + Sync>>>,
    task_receiver: Arc<Mutex<Receiver<Arc<dyn Fn() -> () + Send + Sync>>>>,
    tasks_running: Arc<RwLock<u32>>,
}

impl Threadpool {
    pub fn new(thread_amount: i32) -> Self {
        let mut threads = vec![];
        let tasks_running = Arc::new(RwLock::new(0));

        let channel_tuple = mpsc::channel::<Arc<dyn Fn() -> () + Send + Sync>>();
        let (task_sender, task_receiver) = (
            Mutex::new(channel_tuple.0),
            Arc::new(Mutex::new(channel_tuple.1)),
        );

        for _0 in 0..thread_amount {
            let copy_arc_receiver = task_receiver.clone();
            let copy_tasks_running = tasks_running.clone();

            let thread = thread::Builder::new()
                .spawn(move || loop {
                    let msg_receiver = copy_arc_receiver.lock().unwrap();
                    let message = msg_receiver.recv();

                    *copy_tasks_running.write().unwrap().deref_mut() += 1;
                    match message {
                        Ok(task) => task(),
                        Err(..) => break,
                    };
                    *copy_tasks_running.write().unwrap().deref_mut() -= 1;
                })
                .unwrap();
            threads.push(thread);
        }

        Self {
            threads,
            task_sender,
            task_receiver,
            tasks_running,
        }
    }

    pub fn execute(&self, function: Arc<dyn Fn() -> () + Send + Sync>) -> () {
        let _ = self.task_sender.lock().unwrap().send(function);
    }

    pub fn join(&self) {
        while *self.tasks_running.read().unwrap() != 0 {
            thread::sleep(Duration::from_nanos(10000))
        }
    }

    pub fn finish(self) {
        for handle in self.threads {
            let _ = handle.join();
        }
    }
}
