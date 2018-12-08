use colored::*;

use crate::extend_issue::*;
use crate::extend_issues::*;
use crate::jsprint::JSprint;

const RANK_FIELD: &str = "customfield_14560";

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
    let permalink_padding = issues.permalink_padding(&jsprint.jira);
    let status_padding = issues.status_padding();

    for (assignee, mut user_issues) in issues.group_by_assignee() {
        println!();
        println!("{}", assignee.bold());

        user_issues.sort_unstable_by(|a, b| {
            let rank_a = a.fields.get(RANK_FIELD).map(|v| v.as_str());
            let rank_b = b.fields.get(RANK_FIELD).map(|v| v.as_str());

            rank_a.cmp(&rank_b)
        });

        for issue in user_issues {
            let permalink = &issue.permalink(&jsprint.jira);
            let status = issue.colored_status();
            let summary = issue.display_summary().bold();

            println!(
                "- {:permalink_padding$} ({:status_padding$}) {}",
                permalink,
                status,
                summary,
                permalink_padding = permalink_padding,
                status_padding = status_padding,
            );
        }
    }
}
