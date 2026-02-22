use crate::job::{Job, Status};
use crate::queue::QueueManager;
use chrono::Utc;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub struct TimePriorityEngine {
    queue: Arc<Mutex<QueueManager>>,
    worker_tx: Sender<Job>,
}

impl TimePriorityEngine {
    pub fn new(queue: Arc<Mutex<QueueManager>>, worker_tx: Sender<Job>) -> Self {
        Self { queue, worker_tx }
    }

    /// Starts the Time & Priority Engine in a background thread.
    /// It polls the queue at a set interval for jobs that are ready to execute.
    pub fn start(&self) {
        let queue_clone = Arc::clone(&self.queue);
        let tx_clone = self.worker_tx.clone();

        thread::spawn(move || {
            println!("[Engine] Started polling thread.");
            loop {
                let now = Utc::now().timestamp();

                let mut ready_jobs = Vec::new();
                // Secure the lock briefly to extract ready jobs
                if let Ok(mut q) = queue_clone.lock() {
                    ready_jobs = q.pop_ready(now);
                }

                // Push ready jobs to the worker channel
                for mut job in ready_jobs {
                    job.status = Status::Running;
                    println!(
                        "[Engine] Job {} ('{}') is ready (priority: {}). Dispatching to worker...",
                        job.id, job.description, job.priority
                    );
                    if let Err(e) = tx_clone.send(job) {
                        eprintln!("[Engine] Failed to dispatch job: {}", e);
                    }
                }

                // Poll every 500ms
                thread::sleep(Duration::from_millis(500));
            }
        });
    }
}
