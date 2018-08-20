use commands::get_issue_key_from_number;
use goji::Assignee;
use jsprint::JSprint;

pub fn command(jsprint: &mut JSprint, line: &str) {
    // Parse args
    let args: Vec<&str> = line.split(' ').skip(1).collect();

    if args.is_empty() {
        println!("Assignee and at least one issue number is necessary")
    }

    if args.len() == 1 {
        println!("At least one issue number is necessary")
    }

    // Parse assignee and issue numbers
    let assignee = Assignee::new(args[0]);
    let issues = args[1..]
        .iter()
        .map(|s| get_issue_key_from_number(s).unwrap());

    // Assign issues
    for issue in issues {
        jsprint.jira.issues().assign(&issue, &assignee).unwrap();
    }
}
