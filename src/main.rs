use colored::Colorize;
use std::{env, process::exit};

mod modules;
use modules::{help, path, user};

fn main() {
    if !cfg!(target_os = "linux") {
        println!("Can't run outside Linux.");
        exit(1);
    }

    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    args.len().eq(&0).then(|| args.push("h".to_string()));

    let command = args.remove(0);

    match command.to_lowercase().as_str() {
        "h" | "help" => help::show_help(),
        "u" | "user" => {
            let mut features: Vec<&str> = vec![];
            let mut use_headers = true;

            if args.join("").contains(&"s") { use_headers = false; }
            if args.join("").contains(&"n") { features.push("NAME"); }
            if args.join("").contains(&"u") { features.push("UID"); }
            if args.join("").contains(&"g") { features.push("GROUPS") }

            user::show_user_info(features, use_headers);
        }
        "p" | "path" => {
            let mut simplify = false;
            
            args.join("").contains(&"s").then(|| simplify = true);

            path::show_path(simplify);
        },
        _ => println!("[{}] Invalid command {}", "ERROR".red(), command.yellow()),
    }
}
