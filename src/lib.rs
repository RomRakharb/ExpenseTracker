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

mod date {
    use chrono::prelude::*;

    pub struct Date {
        year: u16,
        month: u8,
        day: u8,
    }

    impl std::fmt::Display for Date {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}-{:02}-{:02}", self.year, self.month, self.day)
        }
    }

    impl Date {
        pub fn now() -> Self {
            let local: DateTime<Local> = Local::now();
            Self {
                year: local.year() as u16,
                month: local.month() as u8,
                day: local.day() as u8,
            }
        }
    }
}

mod expense {
    use std::collections::HashMap;

    use crate::date::Date;

    struct Expense {
        id: u32,
        date: Date,
        category: String,
        description: String,
        amount: u32,
    }

    pub struct Expenses(Vec<Expense>);

    impl Expenses {
        fn add(&mut self, attr: HashMap<String, String>) -> &Self {
            let id = if let Some(expense) = self.0.last() {
                expense.id + 1
            } else {
                1
            };
            let date = Date::now();
            let category = String::new();
            let description = String::new();
            let amount = 1;

            for (key, value) in &attr {
                match key.as_str() {
                    "category" => {}
                    "description" => {}
                    "amount" => {}
                    &_ => todo!(),
                }
            }

            let expense = Expense {
                id,
                date,
                category,
                description,
                amount,
            };

            self.0.push(expense);

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

        fn process(&self, attribute: HashMap<String, String>) {}
    }
}
