use colored::*;
use goji::{SearchOptionsBuilder, Sprint};
use std::cmp;

use crate::extend_sprints::*;
use crate::jsprint::JSprint;

const SPRINTS_WINDOW: usize = 2;

pub fn command(jsprint: &mut JSprint, _line: &str) {
    // Get sprints
    let options = SearchOptionsBuilder::new().build();
    let sprints: Sprints = jsprint.get_sprints(&options).unwrap().collect();

    // Current and future sprints
    let mut current_sprints = sprints
        .iter()
        .filter(|s| s.state != Some("future".to_owned()))
        .collect::<Vec<&Sprint>>();
    let mut future_sprints = sprints
        .iter()
        .filter(|s| s.state == Some("future".to_owned()))
        .collect::<Vec<&Sprint>>();

    current_sprints.sort_unstable_by_key(|s| &s.start_date);
    future_sprints.sort_unstable_by_key(|s| &s.name);

    // Find active sprint and window of current sprints
    let active_pos = current_sprints
        .iter()
        .position(|s| s.state == Some("active".to_owned()));
    let max_index = current_sprints.len();
    let min_pos = active_pos.map_or(0, |i| i.checked_sub(SPRINTS_WINDOW).unwrap_or(0));
    let max_pos = active_pos
        .map(|i| i.checked_add(SPRINTS_WINDOW).unwrap_or(i))
        .map(|i| cmp::min(i, max_index))
        .unwrap_or(max_index);

    // Show sprints
    let sprints_slice = current_sprints[min_pos..max_pos]
        .iter()
        .chain(future_sprints.iter());
    let id_padding = sprints.id_padding();

    for sprint in sprints_slice {
        let state = if sprint.state == Some("active".to_string()) {
            "*"
        } else {
            " "
        };

        println!(
            "{} ({:>id_padding$}) {}",
            state,
            sprint.id.to_string().bold(),
            sprint.name,
            id_padding = id_padding
        );
    }
}
