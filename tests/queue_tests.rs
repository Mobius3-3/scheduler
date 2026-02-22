use scheduler::{job::Job, queue::QueueManager};
use uuid::Uuid;

fn job(exec_time: i64, priority: u8, desc: &str) -> Job {
    Job::new(exec_time, priority, desc, "fn")
}

#[test]
fn push_increases_len() {
    let mut q = QueueManager::new();
    q.push(job(1000, 1, "a"));
    q.push(job(2000, 1, "b"));
    assert_eq!(q.len(), 2);
}

#[test]
fn pop_returns_earliest() {
    let mut q = QueueManager::new();
    q.push(job(3000, 1, "last"));
    q.push(job(1000, 1, "first"));
    q.push(job(2000, 1, "middle"));
    assert_eq!(q.pop().unwrap().description, "first");
    assert_eq!(q.pop().unwrap().description, "middle");
    assert_eq!(q.pop().unwrap().description, "last");
}

#[test]
fn priority_breaks_time_tie() {
    let mut q = QueueManager::new();
    q.push(job(1000, 1, "low"));
    q.push(job(1000, 9, "high"));
    assert_eq!(q.pop().unwrap().description, "high");
}

#[test]
fn peek_does_not_remove() {
    let mut q = QueueManager::new();
    q.push(job(1000, 1, "only"));
    assert_eq!(q.peek().unwrap().description, "only");
    assert_eq!(q.len(), 1);
}

#[test]
fn remove_by_uuid() {
    let mut q = QueueManager::new();
    let j = job(1000, 1, "target");
    let id = j.id;
    q.push(j);
    q.push(job(2000, 1, "other"));
    assert!(q.remove(id).is_some());
    assert_eq!(q.len(), 1);
}

#[test]
fn remove_missing_uuid_returns_none() {
    let mut q = QueueManager::new();
    q.push(job(1000, 1, "job"));
    assert!(q.remove(Uuid::new_v4()).is_none());
    assert_eq!(q.len(), 1);
}

#[test]
fn pop_ready_only_returns_due_jobs() {
    let mut q = QueueManager::new();
    q.push(job(500, 1, "past"));
    q.push(job(1000, 1, "now"));
    q.push(job(9999, 1, "future"));
    let ready = q.pop_ready(1000);
    assert_eq!(ready.len(), 2);
    assert_eq!(q.len(), 1);
}

#[test]
fn pop_on_empty_returns_none() {
    let mut q = QueueManager::new();
    assert!(q.pop().is_none());
}
