use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Task {
    pub task_id: u32,
    pub row: u32,
    pub width: u32,
    pub height: u32,
    pub max_iter: u32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TaskResult {
    pub task_id: u32,
    pub worker_id: String,
    pub row: u32,
    pub data: Vec<u32>,
}
