use std::collections::HashMap;
use std::env;

enum Action {
    Add {
        description: String,
        amount: i32,
        category: Option<String>,
    },
    List,
    Summary(Option<u8>),
    Delete(u32),
}

mod expense {
    struct Expense {
        id: u32,
        date: String,
        category: String,
        description: String,
        amount: u32,
    }

    pub struct Expenses(Vec<Expense>);

    impl Expenses {
        fn add(&mut self) -> &Self {
            let last_id = if let Some(expense) = self.0.last() {
                expense.id + 1
            } else {
                1
            };
            self
        }

        fn list(&mut self) -> &Self {
            self
        }

        fn summary(&mut self) -> &Self {
            self
        }
        fn delete(&mut self) -> &Self {
            self
        }
    }
}

mod file {
    use std::{
        fs::File,
        io::{BufReader, Read, Write},
    };

    pub fn write_file(file_name: &str, contents: String) -> std::io::Result<()> {
        let mut file = File::create(file_name)?;
        file.write_all(contents.as_bytes())?;
        Ok(())
    }

    pub fn read_file(file_name: &str) -> std::io::Result<String> {
        let file = File::open(file_name)?;
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents)?;
        Ok(contents)
    }
}

fn main() {
    let mut args = env::args();
    let mut actions: HashMap<String, String> = HashMap::new();

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "add" | "list" | "summary" | "delete" | "export" => {
                actions.insert("action".to_string(), arg);
            }
            attribute if attribute.starts_with("--") => {
                if let Some(value) = args.next() {
                    actions.insert(arg.trim_start_matches('-').to_string(), value);
                }
            }
            _ => {}
        }
    }

    if let Some(&ref action) = actions.get("action") {
        match action.as_str() {
            "add" => {}
            "list" => {}
            "summary" => {}
            "delete" => {}
            "export" => {}
            _ => {}
        }
    }

    println!("{:?}", actions);
}
