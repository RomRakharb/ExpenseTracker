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

    if let Some(action) = actions.get("action") {
        match action {
            _ => {}
        }
    }

    // match actions["action"]

    println!("{:?}", actions);
}
