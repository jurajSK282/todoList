use serde::{Deserialize, Serialize};
// Make the module public
    // Make the struct public
    #[derive(Serialize, Deserialize)]
    pub struct TodoItem {
        pub id: usize,
        pub title: String, // Make fields public if they need to be accessed directly
        pub completed: bool,
    }

    // Make the struct public
    #[derive(Serialize, Deserialize)]
    pub struct TodoList {
        pub list: Vec<TodoItem>, // Make fields public if they need to be accessed directly
    }

    impl TodoItem {
        // Constructor should be public if it needs to be called from outside
        pub fn new(title: String, id: usize) -> TodoItem {
            TodoItem {
                id: id,
                title: title,
                completed: false,
            }
        }

        // Methods should be public if they need to be called from outside
        pub fn complete(&mut self) {
            self.completed = true;
        }
    }

    impl TodoList {
        // Constructor should be public if it needs to be called from outside
        pub fn new(list: Vec<TodoItem>) -> TodoList {
            TodoList { list }
        }

        // Methods should be public if they need to be called from outside
        pub fn add(&mut self, title: String) {
            let mut i = 0;
            while self.list.iter().any(|item| item.id == i) {
                i += 1;
            }

            self.list.push(TodoItem::new(title, i));
        }

        pub fn complete(&mut self, index: usize) {
            for item in &mut self.list {
                if item.id == index {
                    item.complete();
                    break;
                }
            }
        }

        pub fn remove(&mut self, index: usize) {
            self.list.retain(|item| item.id != index);
        }
        
        pub fn display(&self) {
            if self.list.is_empty() {
                println!("No items in the list");
                return;
            }
            for item in self.list.iter() {
                let status = if item.completed { "completed" } else { "not completed" };
                println!("{} - {} - {}", item.id, item.title, status);
            }
        }
    }