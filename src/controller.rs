use std::{collections::HashMap};
use crate::structs::TodoList;
struct Controls {
    controls: HashMap<&'static str, &'static str>,
}

impl Controls {
    fn new() -> Controls {
        let mut controls = HashMap::new();
        controls.insert("h", "Show controls");
        controls.insert("a", "Add a todo item");
        controls.insert("c", "Complete a todo item");
        controls.insert("d", "Display the todo list");
        controls.insert("q", "Quit the program");
        controls.insert("r", "Remove a todo item");
        Controls { controls }
    }

    fn iter(&self) -> std::collections::hash_map::Iter<&'static str, &'static str> {
        self.controls.iter()
    }
}

pub struct Controller
{
    controls : Controls,
}

impl Controller {
    pub fn new() -> Controller {
        Controller {
            controls: Controls::new(),
        }
    }

    
pub fn show_controls(&self) {
    println!("\nControls:");
    for (key, value) in self.controls.iter() {
        println!("{}: {}", key, value);
    }
}

pub fn process_input(&self, todo_list: &mut TodoList) -> Result<(), &'static str> {
    let mut input = String::new();
    println!("\nEnter a control:");

    if std::io::stdin().read_line(&mut input).is_err() {
        return Err("Failed to read input");
    }

    let input = input.trim();
    match input {
        "a"  => {
            println!("Enter title:");
            let mut title = String::new();
            if std::io::stdin().read_line(&mut title).is_err() {
                return Err("Failed wrong index");
            }
            todo_list.add(title.trim().to_string());
        },
        "c" | "r" => {
            println!("Enter an index:");
            let mut index = String::new();
            if std::io::stdin().read_line(&mut index).is_err() {
                return Err("Failed to read index");
            }
            let index = match index.trim().parse::<usize>() {
                Ok(i) => i,
                Err(_) => return Err("Invalid index"),
            };
            if input == "r" {
                todo_list.remove(index);
            }
            todo_list.complete(index);
        },
        "d" => todo_list.display(),
        "h" => self.show_controls(),
        "q" => return Err("quit"),
        _ => println!("Invalid input"),
    }
    Ok(())
}
}