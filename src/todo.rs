use crate::*;
use csv::ReaderBuilder;
use reqwest::blocking::Client;
use std::error::Error;
use strum::IntoEnumIterator;

/// get_all_todos retrieves the todolist from the server.
/// returns the todolist as a vector of todo items.
pub fn get_all_todos(client: &Client) -> Result<Vec<TodoItem>, Box<dyn Error>> {
    let req_url = format!("{SERVER_ADDR}/getTodos");
    let init_cap = 5;
    let mut todos = Vec::with_capacity(init_cap);

    let data = client.get(req_url).send()?.text()?;

    let mut rdr = ReaderBuilder::new()
        .delimiter(b';')
        .has_headers(false)
        .from_reader(data.as_bytes());
    // from <https://docs.rs/csv/latest/csv/tutorial/index.html#reading-with-serde>
    for result in rdr.deserialize() {
        // We must tell Serde what type we want to deserialize into.
        let record: TodoItem = result?;
        todos.push(record);
    }
    Ok(todos)
}

/// TODO docs
pub fn create_todo(client: &Client) -> Result<(), Box<dyn Error>> {
    println!("Todo name? (required)");
    let name = get_user_input();

    println!("Description? (optional)");
    let desc = get_user_input();

    println!("Todo type? Options:");
    for todo_type in TodoKind::iter() {
        println!("\t{:2}. {todo_type}", todo_type as isize);
    }
    let kind = TodoKind::from_repr(get_number_from_user(false))
        .unwrap_or(TodoKind::Uncategorized);

    let new_todo =
        TodoItem::new(0, name.trim(), desc.trim(), kind, TodoState::Todo);

    client
        .post(format!("{SERVER_ADDR}/createTodo"))
        .form(&new_todo)
        .send()?;

    Ok(())
}

/// TODO docs
pub fn update_todo(todo_id: usize, client: &Client) {
    todo!();
    // let mut wtr = csv::WriterBuilder::new()
    //     .has_headers(false)
    //     .from_writer(std::io::stdout());
    // wtr.serialize(todo_item).unwrap();
    // wtr.flush().unwrap();
}

/// TODO docs
pub fn mark_done(
    todo_id: usize,
    client: &Client,
) -> Result<(), Box<dyn Error>> {
    client
        .put(format!("{SERVER_ADDR}/markTodo/{todo_id}"))
        .send()?;
    Ok(())
}

/// TODO docs
pub fn delete_todo(
    todo_id: usize,
    client: &Client,
) -> Result<(), Box<dyn Error>> {
    client
        .delete(format!("{SERVER_ADDR}/deleteTodo/{todo_id}"))
        .send()?;
    Ok(())
}

/// TODO docs
pub fn get_number_from_user(caller_wants_id_prompt: bool) -> isize {
    if caller_wants_id_prompt {
        println!("\nPlease enter the ID of the item you'd like to change:");
    }

    loop {
        let user_str = get_user_input();
        // match user_str.parse() {
        //     Ok(user_num: usize) => ,
        //     Err(error) => eprintln!("couldn't parse input as num, please try again");
        // }
        if let Ok(user_num) = user_str.parse() {
            return user_num;
        } else {
            eprintln!("couldn't parse input as num, please try again:");
        };
    }
}

/// get_user_input returns a line of input that the user entered into stdin.
fn get_user_input() -> String {
    use std::io;
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("couldn't read from stdin, exiting");
    input = input.trim().to_owned();
    input
}
