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
        todo!()
    }

    fn to_string(&self, index: usize) -> String {
        todo!()
    }

    fn done(&self) -> bool {
        todo!()
    }

    fn set_status(&mut self) {
        todo!()
    }
}

impl Todo for Progress {
    fn new(name: String, description: String) -> Self {
        todo!()
    }

    fn to_string(&self, index: usize) -> String {
        todo!()
    }

    fn done(&self) -> bool {
        todo!()
    }

    fn set_status(&mut self) {
        todo!()
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
    todo!()
}

fn main() {
    // There's cleaner way to write this code, but not now
    let mut check_list: Vec<Check> = Vec::new();
    let mut progress_list: Vec<Progress> = Vec::new();

    loop {
        println!("What do you want to do?");
        println!("1. Add new todo");
        println!("2. Edit todo");
        println!("3. Show todo list");
        println!("4. Exit");

        todo!()
    }
}
