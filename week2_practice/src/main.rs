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

    fn to_string(&self, index: usize) -> String;
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

    fn to_string(&self, index: usize) -> String {
        let status = if self.done() { "[X]" } else { "[ ]" };
        format!(
            "{}. {} {}:\n\t{}",
            index, status, self.name, self.description
        )
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

    fn to_string(&self, index: usize) -> String {
        let status = if self.done() { "[X]" } else { "[ ]" };
        format!(
            "{}. {} {}:\n\t{}\n\tProgress: {:3}%",
            index, status, self.name, self.description, self.progress
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

fn create_todo<T: Todo>(todo_list: &mut Vec<T>) {
    println!("Name: ");
    let name = stdin();

    println!("Description: ");
    let description = stdin();

    let check = T::new(name, description);
    todo_list.push(check);
}

fn main() {
    // let mut todo_list: Vec<TodoItem> = Vec::new();
    // let mut todo_list: Vec<TodoItem> = Vec::new();
    let mut check_list: Vec<Check> = Vec::new();
    let mut progress_list: Vec<Progress> = Vec::new();

    loop {
        let check_list_len = check_list.len();
        let progress_list_len = progress_list.len();
        let todo_list_len = check_list_len + progress_list_len;

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
                        create_todo(&mut check_list);
                    }
                    "2" => {
                        create_todo(&mut progress_list);
                    }
                    _ => println!("Invalid input"),
                }
            }
            "2" => {
                println!("Which todo do you want to edit? [1-{}]", todo_list_len);
                let input = stdin();

                match string_to_u8(input) {
                    Some(index) => {
                        let index = index - 1;

                        if index <= todo_list_len as u8 {
                            if index < check_list_len as u8 {
                                let check = &mut check_list[index as usize];
                                check.set_status();
                            } else {
                                let progress = &mut progress_list[index as usize - check_list_len];
                                progress.set_status();
                            }
                        } else {
                            println!("Invalid index");
                        }
                    }
                    None => println!("Invalid input"),
                }
            }
            "3" => {
                for (index, check) in check_list.iter().enumerate() {
                    println!("{}", check.to_string(index + 1));
                }
                for (index, progress) in progress_list.iter().enumerate() {
                    println!("{}", progress.to_string(index + 1 + check_list_len));
                }
            }
            "4" => break,
            _ => println!("Invalid input"),
        }
    }
}
