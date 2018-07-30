use colored::*;
use extend_issue::*;
use extend_issues::*;
use jsprint::JSprint;

pub fn command(jsprint: &mut JSprint, _line: &str) {
    // Get current active sprint
    let sprint = jsprint.current_sprint.clone().unwrap();

    println!("Displaying sprint {}", sprint.name);

    // Search for issues
    let issues = jsprint.get_issues(&sprint);

    if issues.is_empty() {
        println!("No issues found for sprint {}", sprint.name);
        return;
    }

    // Calculate padding
    let key_padding = issues.key_padding();
    let permalink_padding = issues.permalink_padding(&jsprint.jira);
    let status_padding = issues.status_padding();

    for (assignee, mut user_issues) in issues.group_by_assignee() {
        println!();
        println!("{}", assignee.bold());

        user_issues.sort_unstable_by(|a, b| a.id.cmp(&b.id));

        for issue in user_issues {
            let key = &issue.key.bold();
            let permalink = &issue.permalink(&jsprint.jira);
            let status = issue.colored_status();
            let summary = issue.display_summary().bold();

            println!(
                "{:status_padding$} - {:key_padding$} ({:permalink_padding$}) {}",
                status,
                key,
                permalink,
                summary,
                status_padding = status_padding,
                key_padding = key_padding,
                permalink_padding = permalink_padding,
            );
        }
    }
}
