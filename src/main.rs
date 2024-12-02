use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, BufReader, BufWriter};

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    id: u32,
    title: String,
    completed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct TodoList {
    tasks: Vec<Task>,
}

impl TodoList {
    fn new() -> Self {
        Self { tasks: Vec::new() }
    }

    fn load_from_file(filename: &str) -> io::Result<Self> {
        File::open(filename)
            .map(BufReader::new)
            .and_then(|reader| serde_json::from_reader(reader).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e)))
    }

    fn save_to_file(&self, filename: &str) -> io::Result<()> {
        File::create(filename)
            .map(BufWriter::new)
            .and_then(|writer| serde_json::to_writer(writer, &self).map_err(|e| io::Error::new(io::ErrorKind::Other, e)))
    }

    fn add_task(&mut self, title: String) {
        let id = (self.tasks.len() as u32) + 1;
        self.tasks.push(Task {
            id,
            title,
            completed: false,
        });
        println!("Завдання додано!");
    }

    fn delete_task(&mut self, id: u32) {
        if let Some(index) = self.tasks.iter().position(|task| task.id == id) {
            self.tasks.remove(index);
            self.reassign_ids();
            println!("Завдання видалено!");
        } else {
            println!("Завдання з таким ID не знайдено.");
        }
    }

    fn reassign_ids(&mut self) {
        for (index, task) in self.tasks.iter_mut().enumerate() {
            task.id = (index as u32) + 1;
        }
    }

    fn edit_task(&mut self, id: u32, new_title: String) {
        match self.tasks.iter_mut().find(|task| task.id == id) {
            Some(task) => {
                task.title = new_title;
                println!("Завдання оновлено!");
            }
            None => println!("Завдання з таким ID не знайдено."),
        }
    }

    fn mark_completed(&mut self, id: u32) {
        match self.tasks.iter_mut().find(|task| task.id == id) {
            Some(task) => {
                task.completed = true;
                println!("Завдання позначено як виконане!");
            }
            None => println!("Завдання з таким ID не знайдено."),
        }
    }

    fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("Список завдань порожній.");
        } else {
            println!("Список завдань:");
            for task in &self.tasks {
                println!(
                    "{}. {} [{}]",
                    task.id,
                    task.title,
                    if task.completed { "Виконано" } else { "Не виконано" }
                );
            }
        }
    }
}

fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Помилка введення");
    input.trim().to_string()
}

fn main() {
    let filename = "tasks.json";
    let mut todo_list = TodoList::load_from_file(filename).unwrap_or_else(|_| TodoList::new());

    loop {
        todo_list.list_tasks();

        println!(
            "\nМеню:\n1. Додати завдання\n2. Видалити завдання\n3. Редагувати завдання\n4. Позначити завдання виконаним\n5. Зберегти та вийти"
        );

        match get_input("Оберіть опцію:").as_str() {
            "1" => {
                let title = get_input("Введіть назву завдання:");
                todo_list.add_task(title);
            }
            "2" => {
                let id = get_input("Введіть ID завдання для видалення:")
                    .parse::<u32>()
                    .unwrap_or_default();
                todo_list.delete_task(id);
            }
            "3" => {
                let id = get_input("Введіть ID завдання для редагування:")
                    .parse::<u32>()
                    .unwrap_or_default();
                let new_title = get_input("Введіть нову назву завдання:");
                todo_list.edit_task(id, new_title);
            }
            "4" => {
                let id = get_input("Введіть ID завдання для позначення виконаним:")
                    .parse::<u32>()
                    .unwrap_or_default();
                todo_list.mark_completed(id);
            }
            "5" => {
                if let Err(e) = todo_list.save_to_file(filename) {
                    eprintln!("Помилка збереження: {}", e);
                } else {
                    println!("Список завдань збережено. До побачення!");
                }
                break;
            }
            _ => println!("Невірний вибір. Спробуйте ще раз."),
        }
    }
}