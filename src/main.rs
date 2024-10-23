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
            "add" => Action::Add {
                category: actions.get("category").cloned(),
                description: actions
                    .get("description")
                    .cloned()
                    .ok_or_else(|| "Description required".to_string())?,
                amount: {
                    let amount = actions
                        .get("amount")
                        .ok_or_else(|| "Amount required".to_string())?;
                    amount.parse()?
                },
            },
            "list" => Action::List {
                month: actions.get("month").and_then(|s| s.parse().ok()),
                year: actions.get("year").and_then(|s| s.parse().ok()),
                category: actions.get("category").cloned(),
            },
            "summary" => Action::Summary {
                month: actions.get("month").and_then(|s| s.parse().ok()),
                year: actions.get("year").and_then(|s| s.parse().ok()),
                category: actions.get("category").cloned(),
            },
            "delete" => Action::Delete({
                let id = actions.get("id").ok_or_else(|| "ID required".to_string())?;
                id.parse()?
            }),
            _ => todo!(),
        };
    }

    println!("{:?}", actions);
    Ok(())
}
