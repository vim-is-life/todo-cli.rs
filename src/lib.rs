/// Type definitions for todo app
use serde::{Deserialize, Serialize};
use serde_repr::*;
use strum_macros::{Display, EnumIter, FromRepr};

pub mod todo;

const SERVER_ADDR: &str = "http://localhost:9000/api";
// right now we won't worry about dynamically getting the address, i just want a
// global that i can have and do work with for now
// static base_addr: String = std::env::var("SERVER_ADDR").unwrap_or(default_addr.to_owned());

/// TodoItem represents a todo item in our list of things to do.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TodoItem {
    pub todo_id: usize,
    pub name: String,
    pub desc: String,
    pub kind: TodoKind,
    pub state: TodoState,
}

impl TodoItem {
    pub fn new(id: usize, name: &str, desc: &str, kind: TodoKind, state: TodoState) -> Self {
        Self {
            todo_id: id,
            name: name.to_owned(),
            desc: desc.to_owned(),
            kind,
            state,
        }
    }
}

/// TodoKind represents what kind of task a todo item is.
#[derive(Copy, Clone, Debug, Serialize_repr, Deserialize_repr)]
#[repr(isize)]
#[derive(Display, EnumIter, FromRepr)]
pub enum TodoKind {
    Uncategorized = -1,
    Project,
    Homework,
    Reading,
    Study,
}

/// TodoState represents the completion state of a task in the todo list.
#[derive(Clone, Debug, Serialize_repr, Deserialize_repr)]
#[repr(isize)]
#[derive(Display, EnumIter, FromRepr)]
pub enum TodoState {
    InProgress = -1,
    Todo,
    Done,
}
