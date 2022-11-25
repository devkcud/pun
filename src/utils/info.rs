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
    println!("{}:{}",
        extra_title,
        sep.to_string() + &msg.join(sep).red().to_string()[..],
    );
}

pub fn help_module() {
    show_description("Shows a help menu.");
    show_usage("help", vec!["<module>"]);
    show_extra(
        "Examples",
        vec!["", "components", "user", "path"],
        &format!("\n  {} {} {} ",
            "$".magenta().bold(),
            get_executable_name().green(),
            "help".red(),
        )[..]
    );
    show_extra("Aliases", vec!["h", "help"], " ");
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
    show_extra("Aliases", vec!["p", "path"], " ");
}

pub fn user_module() {
    show_description("Show user information.");
    show_usage("user", vec!["[nugs]"]);
    show_extra(
        "Examples",
        vec!["", "s", "ng", "nus"],
        &format!("\n  {} {} {} ",
            "$".magenta().bold(),
            get_executable_name().green(),
            "user".red(),
        )[..]
    );
}

pub fn components_module() {
    show_description("Show components temperature.");
    show_usage("components", vec![]);
    show_extra("Examples", vec![""], "");
    show_extra("Aliases", vec!["c", "comp", "components"], " ");
}