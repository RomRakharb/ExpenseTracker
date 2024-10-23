use std::collections::HashMap;
use std::env;

enum Action {
    Add {
        category: Option<String>,
        description: String,
        amount: u32,
    },
    List {
        month: Option<u8>,
        year: Option<u16>,
        category: Option<String>,
    },
    Summary {
        month: Option<u8>,
        year: Option<u16>,
        category: Option<String>,
    },
    Delete(u32),
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
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
        let action_enum = match action.as_str() {
            "add" => todo!(),
            "list" => todo!(),
            "summary" => todo!(),
            "delete" => todo!(),
            "export" => todo!(),
            _ => todo!(),
        };
    }

    println!("{:?}", actions);
    Ok(())
}
