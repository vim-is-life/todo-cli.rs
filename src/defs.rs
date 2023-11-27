/// Type definitions for todo app
// mod lib;
use serde::{Deserialize, Serialize};

/// TodoItem represents a todo item in our list of things to do
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TodoItem {
    pub todo_id: usize,
    pub name: String,
    pub desc: String,
    pub kind: TodoKind,
    pub state: TodoState,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TodoKind {
    Uncategorized = -1,
    Project,
    Homework,
    Reading,
    Study,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TodoState {
    InProgress = -1,
    Todo,
    Done,
}
