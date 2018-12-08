use std::collections::BTreeMap;

use crate::jsprint::JSprint;

pub fn command(jsprint: &mut JSprint, _line: &str) {
    // Get current active sprint
    let sprint = jsprint.current_sprint.clone().unwrap();

    println!("Displaying sprint {}", sprint.name);

    // Search for issues
    let issues = jsprint.get_issues(&sprint);

    // Calculate labels count
    let label_count = issues
        .iter()
        .flat_map(|issue| issue.labels())
        .map(|label| {
            jsprint
                .settings
                .labels_mapping
                .get(&label)
                .unwrap_or(&format!("<{}>", label))
                .to_owned()
        })
        .fold(BTreeMap::new(), |mut acc, label| {
            *acc.entry(label).or_insert(0) += 1;

            acc
        });

    // Print results
    println!("{:#?}", label_count);
}
