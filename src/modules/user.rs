use itertools::Itertools;
use sysinfo::{System, SystemExt, UserExt};
use term_grid::{Cell, Direction::LeftToRight, Filling::Spaces, Grid, GridOptions};

pub fn show_user_info(mut features: Vec<&str>, use_headers: bool) {
    let mut output = Grid::new(GridOptions {
        direction: LeftToRight,
        filling: Spaces(5),
    });

    let mut add_to_output = |info: &str| output.add(Cell::from(info));

    if features.len() == 0 {
        features = vec!["NAME", "UID", "GROUPS"];
    }

    let features: Vec<&str> = features.into_iter().unique().collect();

    if use_headers {
        for feature in features.iter() {
            add_to_output(feature);
        }
    }

    for user in System::new_all().users().iter() {
        for feature in features.iter() {
            match feature {
                &"NAME"   => add_to_output(user.name()),
                &"UID"    => add_to_output(user.id().to_string().as_str()),
                &"GROUPS" => add_to_output(user.groups().join(", ").as_str()),
                _ => (),
            }
        }
    }

    println!("{}", output.fit_into_columns(features.len()));
}
