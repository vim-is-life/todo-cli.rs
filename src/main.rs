use crate::todo::*;
use reqwest::blocking::Client;
use todo_cli_rs::*;

fn main() {
    let client = Client::new();

    loop {
        display_todos(&client);
        display_prompt();

        println!("Please enter your choice: ");
        // let user_choice: isize =
        //     get_user_input().parse().unwrap_or_else(|_err| {
        //         println!("Couldn't parse input as integer, please try again.");
        //         -1
        //     });
        let user_choice = get_number_from_user();

        match user_choice {
            1 => create_todo(&client).unwrap(),
            2 => update_todo(get_number_from_user().try_into().unwrap()),
            // 3 => mark_done(0),
            // 4 => delete_todo(0),
            _ => (),
        }
    }
}

/// display_todos prints all the todos to the console
/// takes in a reference to the todolist (which is vector of todo items)
fn display_todos(client: &Client) {
    let all_todo_items = get_all_todos(client).unwrap_or_else(|err| {
        eprintln!("couldn't get the todos because of error:\n\t{err}");
        std::process::exit(1);
    });

    let w1 = 4;
    let w2 = 25;
    let w3 = 50;
    let w4 = 20;
    let w5 = 20;

    // print the header
    println!(
        "{:w1$}{:w2$}{:w3$}{:w4$}{:w5$}",
        "id", "name", "description", "kind", "state",
    );
    println!(
        "{:w1$}{:w2$}{:w3$}{:w4$}{:w5$}",
        "==", "====", "===========", "====", "====="
    );

    // print the rows with todo items
    for todo_item in all_todo_items {
        println!(
            "{:<w1$}{:w2$}{:w3$}{:w4$}{:w5$}",
            todo_item.todo_id,
            todo_item.name,
            todo_item.desc,
            todo_item.kind,
            todo_item.state,
        );
    }
}

/// display_prompt shows the menu prompt to the user.
fn display_prompt() {
    // create, update, marktodo, delete
    let msg = "\
What would you like to do?
\t1. Create a new todo item.
\t2. Update an existing todo item.
\t3. Mark a todo item as done.
\t4. Delete a todo item.";
    println!("\n{msg}");
}
