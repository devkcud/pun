use std::vec;

use colored::Colorize;
use super::exec::get_executable_name;

fn show_description(description: &str) {
    println!("# {}", description.magenta());
}

fn show_usage(module_name: &str, args: Vec<&str>) {
    println!("Usage: {} {} {}",
        get_executable_name().green(),
        module_name.red(),
        args.join(" ").red(),
    );
}

fn show_extra(extra_title: &str, msg: Vec<&str>, sep: &str) {
    println!("{}: {}",
        extra_title,
        sep.to_string() + &msg.join(sep).red().to_string()[..],
    );
}

pub fn help_module() {
    show_description("Shows a help menu.");
    show_usage("help", vec!["<module>"]);
    show_extra(
        "Examples",
        vec!["", "path", "user", "p", "u"],
        &format!("\n  {} {} {} ",
            "$".magenta().bold(),
            get_executable_name().green(),
            "help".red(),
        )[..]
    );
}

pub fn path_module() {
    show_description("Shows the $PATH variable in a human-readable way.");
    show_usage("path", vec!["[s]"]);
    show_extra(
        "Examples",
        vec!["", "s"],
        &format!("\n  {} {} {} ",
            "$".magenta().bold(),
            get_executable_name().green(),
            "path".red(),
        )[..]
    );
}

pub fn user_module() {
    show_description("Show user information.");
    show_usage("user", vec!["[nug-]"]);
    show_extra(
        "Examples",
        vec!["-", "ng", "nu-"],
        &format!("\n  {} {} {} ",
            "$".magenta().bold(),
            get_executable_name().green(),
            "user".red(),
        )[..]
    );
}