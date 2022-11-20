use std::process::exit;

use colored::Colorize;
use term_grid::{Cell, Grid, GridOptions};

pub fn show_path(simplify: bool) {
    if simplify {
        println!("{}", env!("PATH").split(":").collect::<Vec<&str>>().join("\n"));
        exit(0);
    }

    let mut output = Grid::new(GridOptions {
        direction: term_grid::Direction::LeftToRight,
        filling: term_grid::Filling::Spaces(1),
    });

    let mut path: Vec<&str> = env!("PATH").split(":").collect();
    let size: usize = path.len();
    
    path.sort_by(|a, b| a.cmp(b));

    for (i, location) in path.iter().enumerate() {
        output.add(Cell::from((i + 1).to_string().bright_black().to_string()));
        output.add(Cell::from(location.yellow().to_string()));
    }
    
    println!("{}\nSize: {}", output.fit_into_columns(2), size.to_string().bright_black());
}
