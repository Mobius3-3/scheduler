use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::HashMap;
use std::{thread, time::Duration};

use crate::job::Job;
use crate::queue::QueueManager;

/// Type alias for a function pointer that takes no arguments and returns nothing
type JobFn = fn();

pub struct Worker {
    registry: HashMap<String, JobFn>,
}

impl Worker {
    /// Initialize a new worker with an empty registry
    pub fn new() -> Self {
        Self {
            registry: HashMap::new(),
        }
    }

    /// Register a function string to a concrete function pointer
    pub fn register(&mut self, name: &str, f: JobFn) {
        self.registry.insert(name.to_string(), f);
    }

    /// The execution engine: looks up the string in the map and calls the function
    pub fn run_job(&self, job: &Job) {
        if let Some(func) = self.registry.get(&job.function) {
            println!("[Worker] Executing: {}", job.function);
            func(); // Execute the function pointer
        } else {
            eprintln!("[Worker] Error: No function registered for '{}'", job.function);
        }
    }

    /// Starts a simple polling loop to process jobs from the queue
    pub fn start(&self, queue: &mut QueueManager) {
        loop {
                    let now = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs() as i64;

                    let ready_jobs = queue.pop_ready(now);

                    for job in ready_jobs {
                        self.run_job(&job);
                    }

                    // Prevent 100% CPU usage during idle
                    thread::sleep(Duration::from_millis(100));
            }
    }

    /// Processes all currently ready jobs once and returns (for testing/manual polling)
    pub fn process_once(&self, queue: &mut QueueManager) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        let ready_jobs = queue.pop_ready(now);
        for job in ready_jobs {
            self.run_job(&job);
        }
    }
}

// --- Task Functions ---

pub fn send_email() {
    println!("📧 [Task] Sending email...");
    // Logic for sending email here
}

pub fn backup_db() {
    println!("🗄️ [Task] Backing up database...");
    // Logic for DB backup here
}