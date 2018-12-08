use crate::commands::get_issue_key_from_number;
use crate::jsprint::JSprint;

pub fn command(jsprint: &mut JSprint, line: &str) {
    // Get issue numbers
    let issues = line
        .split_whitespace()
        .skip(1)
        .map(|s| get_issue_key_from_number(s).unwrap())
        .collect::<Vec<String>>();

    if issues.is_empty() {
        println!("At least one issue number is needed");
        return;
    }

    jsprint.jira.backlog().put(issues).unwrap();
}
