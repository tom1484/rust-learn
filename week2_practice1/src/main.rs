mod template;

struct Check {
    name: String,
    description: String,
    done: bool,
}

struct Progress {
    name: String,
    description: String,
    progress: u8,
}

trait Todo {
    fn new(name: String, description: String) -> Self;

    fn to_string(&self) -> String;
    fn done(&self) -> bool;

    fn set_status(&mut self);
}

impl Todo for Check {
    fn new(name: String, description: String) -> Self {
        Check {
            name,
            description,
            done: false,
        }
    }

    fn to_string(&self) -> String {
        let status = if self.done() { "[X]" } else { "[ ]" };
        format!("{} {}:\n\t{}", status, self.name, self.description)
    }

    fn done(&self) -> bool {
        self.done
    }

    fn set_status(&mut self) {
        loop {
            println!("Set status to done? [y/n] ");
            let input = stdin();

            match input.as_str() {
                "y" => {
                    self.done = true;
                    break;
                }
                "n" => {
                    self.done = false;
                    break;
                }
                _ => println!("Invalid input"),
            }
        }
    }
}

impl Todo for Progress {
    fn new(name: String, description: String) -> Self {
        Progress {
            name,
            description,
            progress: 0,
        }
    }

    fn to_string(&self) -> String {
        let status = if self.done() { "[X]" } else { "[ ]" };
        format!(
            "{} {}:\n\t{}\n\tProgress: {:3}%",
            status, self.name, self.description, self.progress
        )
    }

    fn done(&self) -> bool {
        self.progress == 100
    }

    fn set_status(&mut self) {
        loop {
            println!("Set status progress [0-100] ");
            let input = stdin();

            match string_to_u8(input) {
                Some(progress) => {
                    self.progress = progress;
                    break;
                }
                None => println!("Invalid input"),
            }
        }
    }
}

enum TodoItem {
    Check(Check),
    Progress(Progress),
}

fn stdin() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap_or(0);
    input.trim().to_string()
}

fn string_to_u8(input: String) -> Option<u8> {
    match input.parse::<u8>() {
        Ok(progress) => Some(progress),
        Err(_) => None,
    }
}

fn main() {
    let mut todo_list: Vec<TodoItem> = Vec::new();

    loop {
        println!("What do you want to do?");
        println!("1. Add new todo");
        println!("2. Edit todo");
        println!("3. Show todo list");
        println!("4. Exit");

        let input = stdin();

        match input.as_str() {
            "1" => {
                println!("What kind of todo do you want to add?");
                println!("1. Check");
                println!("2. Progress");

                let input = stdin();

                match input.as_str() {
                    "1" => {
                        println!("Name: ");
                        let name = stdin();

                        println!("Description: ");
                        let description = stdin();

                        let check = Check::new(name, description);
                        todo_list.push(TodoItem::Check(check));
                    }
                    "2" => {
                        println!("Name: ");
                        let name = stdin();

                        println!("Description: ");
                        let description = stdin();

                        let progress = Progress::new(name, description);
                        todo_list.push(TodoItem::Progress(progress));
                    }
                    _ => println!("Invalid input"),
                }
            }
            "2" => {
                println!(
                    "Which todo do you want to edit? [0-{}]",
                    todo_list.len() - 1
                );
                let input = stdin();

                match string_to_u8(input) {
                    Some(index) => {
                        if index < todo_list.len() as u8 {
                            let todo = &mut todo_list[index as usize];

                            match todo {
                                TodoItem::Check(check) => {
                                    check.set_status();
                                }
                                TodoItem::Progress(progress) => {
                                    progress.set_status();
                                }
                            }
                        } else {
                            println!("Invalid index");
                        }
                    }
                    None => println!("Invalid input"),
                }
            }
            "3" => {
                for todo in todo_list.iter() {
                    match todo {
                        TodoItem::Check(check) => println!("{}", check.to_string()),
                        TodoItem::Progress(progress) => println!("{}", progress.to_string()),
                    }
                }
            }
            "4" => break,
            _ => println!("Invalid input"),
        }
    }
}
