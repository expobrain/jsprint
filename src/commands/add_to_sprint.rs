use commands::get_issue_key_from_number;
use jsprint::JSprint;

pub fn command(jsprint: &mut JSprint, line: &str) {
    // Parse args
    let args = line
        .split(' ')
        .skip(1)
        .map(|s| get_issue_key_from_number(s).unwrap())
        .collect::<Vec<String>>();

    if args.is_empty() {
        println!("At least one issue number is needed")
    }

    // Move issues
    let sprint_id = jsprint.current_sprint.clone().unwrap().id;

    let _ = jsprint.jira.sprints().move_issues(sprint_id, args);
}
