pub mod structs;
mod controller;

use std::fs::File;
use std::io::Write;

use structs::TodoList;
use controller::Controller;

fn read_json() -> Result<TodoList, Box<dyn std::error::Error>> {
    let file = std::fs::File::open("list.json")?;
    let data: TodoList = serde_json::from_reader(file)?;
    Ok(data)
}

pub fn save_to_file(list : &TodoList) -> std::io::Result<()> {
    let json = serde_json::to_string(list)?;
    let mut file = File::create("list.json")?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

pub fn initialize_todo_list() -> TodoList {
    let todo_list_result = read_json();
    if let Err(e) = &todo_list_result {
        println!("Failed to read JSON: {}", e);
    } else {
        println!("Successfully read JSON");
    }
    let todo_list = todo_list_result.unwrap_or_else(|_| TodoList::new(Vec::new()));
    todo_list
}

fn main() -> Result<(), std::io::Error> {
    let controller = Controller::new();
    controller.show_controls();

    let mut todo_list = initialize_todo_list();

    loop {
        match controller.process_input(&mut todo_list) {
            Ok(_) => continue,
            Err("quit") => {
                let res = save_to_file(&todo_list);
                match res {
                    Ok(_) => { 
                        println!("Successfully saved to file");
                        return Ok(());
                    },
                    Err(e) => { 
                        println!("Failed to save to file: {}", e);
                        return Err(e);
                    }
                }
            },
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}