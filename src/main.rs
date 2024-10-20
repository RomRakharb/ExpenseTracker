use std::collections::HashMap;
use std::env;

enum Action {
    Add(String, u32),
    List,
    Summary(Option<u8>),
    Delete(u32),
}

fn main() {
    let mut args = env::args();
    let mut actions: HashMap<&str, String> = HashMap::new();

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "add" | "list" | "summary" | "delete" | "export" => {}
            "--description" => {}
            "--amount" => {}
            "--id" => {}
            "--month" => {}
            "--category" => {}
            _ => {}
        }
    }
    println!("{:?}", actions);
}
