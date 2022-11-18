#[path = "../exec_utils.rs"]
mod exec_utils;
use exec_utils::get_exec_name;

use colored::Colorize;
use std::{env::args, process::exit};
use term_grid::{Cell, Grid, GridOptions};

fn help_default() {
    println!(
        "Usage: {} {}",
        get_exec_name().green().bold(),
        "help <module>".magenta().bold()
    );

    println!(
        "Available modules: {}",
        vec!["help", "path", "user"].join(", ").magenta().bold()
    );
}

fn help_help() {
    let mut output_grid = Grid::new(GridOptions {
        direction: term_grid::Direction::LeftToRight,
        filling: term_grid::Filling::Spaces(5),
    });

    output_grid.add(Cell::from("Description".blue().bold().to_string()));
    output_grid.add(Cell::from("Shows help menu."));

    output_grid.add(Cell::from("Usage".blue().bold().to_string()));
    output_grid.add(Cell::from(format!(
        "{} {}",
        get_exec_name().green().bold(),
        "help <module>".magenta().bold()
    )));

    output_grid.add(Cell::from("Available modules".blue().bold().to_string()));
    output_grid.add(Cell::from(
        vec!["help", "path", "user"]
            .join(", ")
            .magenta()
            .bold()
            .to_string(),
    ));

    println!("{}", output_grid.fit_into_columns(2));
}

fn help_path() {
    let mut output_grid = Grid::new(GridOptions {
        direction: term_grid::Direction::LeftToRight,
        filling: term_grid::Filling::Spaces(5),
    });

    output_grid.add(Cell::from("Description".blue().bold().to_string()));
    output_grid.add(Cell::from("Shows the path in a human-readable way."));

    output_grid.add(Cell::from("Usage".blue().bold().to_string()));
    output_grid.add(Cell::from(format!(
        "{} {}",
        get_exec_name().green().bold(),
        "path".magenta().bold()
    )));

    println!("{}", output_grid.fit_into_columns(2));
}

fn help_user() {
    let mut output_grid = Grid::new(GridOptions {
        direction: term_grid::Direction::LeftToRight,
        filling: term_grid::Filling::Spaces(5),
    });

    output_grid.add(Cell::from("Description".blue().bold().to_string()));
    output_grid.add(Cell::from("Shows user information."));

    output_grid.add(Cell::from("Usage".blue().bold().to_string()));
    output_grid.add(Cell::from(format!(
        "{} {}",
        get_exec_name().green().bold(),
        format!("user [{} {{ {} }}]", "OPTIONS".italic(), "nug-".italic())
            .magenta()
            .bold()
    )));

    output_grid.add(Cell::from("Example".bold().blue().to_string()));
    output_grid.add(Cell::from(format!("{} user ng", get_exec_name())));

    println!("{}", output_grid.fit_into_columns(2));
}

pub fn show_help() {
    let mut args: Vec<String> = args().collect();
    args.remove(0);

    if args.len() == 0 {
        help_default();
        exit(0);
    }

    args.remove(0);

    let command = args.pop().unwrap();

    match command.as_str() {
        "h" | "help" => help_help(),
        "p" | "path" => help_path(),
        "u" | "user" => help_user(),
        _ => help_default(),
    }
}
