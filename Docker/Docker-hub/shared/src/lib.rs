use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub task_id: u32,
    pub row: u32,
    pub width: u32,
    pub height: u32,
    pub max_iter: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TaskResult {
    pub worker_id: String,
    pub row: u32,
    pub data: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LogMessage {
    pub source: String,
    pub level: String,
    pub message: String,
    pub timestamp: u64,
}
