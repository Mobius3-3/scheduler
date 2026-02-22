use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Status {
    Pending,
    Running,
    Success,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Job {
    pub id: Uuid,
    pub execution_time: i64,
    pub priority: u8,
    pub description: String,
    pub function: String,
    pub status: Status,
}

impl Job {
    pub fn new(
        execution_time: i64,
        priority: u8,
        description: impl Into<String>,
        function: impl Into<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            execution_time,
            priority,
            description: description.into(),
            function: function.into(),
            status: Status::Pending,
        }
    }
}

impl PartialEq for Job {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Job {}

impl PartialOrd for Job {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Job {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .execution_time
            .cmp(&self.execution_time)
            .then(self.priority.cmp(&other.priority))
    }
}
