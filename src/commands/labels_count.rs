use command::Command;
use jsprint::JSprint;
use std::collections::BTreeMap;

fn exec(jsprint: &mut JSprint, _line: &str) {
    // Get current active sprint
    let sprint = jsprint.get_active_sprint().unwrap();

    println!("Displaying sprint {}", sprint.name);

    // Search for issues
    let issues = jsprint.get_issues(&sprint).unwrap();

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

pub fn get_command() -> Command {
    Command {
        exec: Box::new(exec),
    }
}
