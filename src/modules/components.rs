use sysinfo::{ComponentExt, System, SystemExt};
use term_grid::{Grid, GridOptions, Cell};

pub fn show_components_info() {
    let mut output = Grid::new(GridOptions { direction: term_grid::Direction::LeftToRight, filling: term_grid::Filling::Spaces(1) });

    for component in System::new_all().components() {
        output.add(Cell::from(component.label()));
        output.add(Cell::from(format!("{}°C", component.temperature())));
    }
    
    println!("{}", output.fit_into_columns(2));
}