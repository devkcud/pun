use std::env::args;

#[path ="../utils.rs"]
mod utils;
use utils::info;

pub fn show_help() {
    match args().collect::<Vec<String>>().pop().unwrap().as_str() {
        "h" | "help" => info::help_module(),
        "p" | "path" => info::path_module(),
        "u" | "user" => info::user_module(),
        "c" | "comp" | "components" => info::components_module(),
        _ => {}
    }
}
