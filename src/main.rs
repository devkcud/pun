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

            if args.len() != 0 {
                args.join("")
                    .chars()
                    .collect::<Vec<char>>()
                    .iter()
                    .for_each(|c| match c {
                        'n' => features.push("NAME"),
                        'u' => features.push("UID"),
                        'g' => features.push("GROUPS"),
                        '-' => use_headers = false,
                        _ => (),
                    });
            }

            user::show_user_info(features, use_headers);
        }
        "p" | "path" => path::show_path(),
        _ => println!("[{}] Invalid command {}", "ERROR".red(), command.yellow()),
    }
}
