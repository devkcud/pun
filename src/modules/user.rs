use itertools::Itertools;
use sysinfo::{System, SystemExt, UserExt};
use term_grid::{Cell, Direction::LeftToRight, Filling::Spaces, Grid, GridOptions};

pub fn show_user_info(mut features: Vec<&str>, use_headers: bool) {
    let mut output = Grid::new(GridOptions {
        direction: LeftToRight,
        filling: Spaces(5),
    });
    let mut add_to_output = |info: &str| output.add(Cell::from(info));

    features
        .len()
        .le(&0)
        .then(|| features = vec!["NAME", "UID", "GROUPS"]);

    let features: Vec<&str> = features.into_iter().unique().collect();
    use_headers.then(|| features.iter().for_each(|f| add_to_output(f)));

    System::new_all().users().iter().for_each(|u| {
        features.iter().for_each(|f| match f {
            &"NAME" => add_to_output(u.name()),
            &"UID" => add_to_output(u.id().to_string().as_str()),
            &"GROUPS" => add_to_output(u.groups().join(", ").as_str()),
            _ => (),
        });
    });

    println!("{}", output.fit_into_columns(features.len()));
}
