mod defs;
use defs::*;

fn main() {
    println!("{}", TodoKind::Project as isize);
    println!("{}", TodoState::Done as isize);

    let t = TodoItem {
        todo_id: 0,
        name: String::from("test name"),
        desc: String::from("this is a description"),
        kind: TodoKind::Homework,
        state: TodoState::InProgress,
    };

    println!("{:?}", t);
}
