use colored::*;
use extend_issue::*;
use extend_issues::*;
use jsprint::JSprint;

const REVIEW_PADDING: usize = 6;

pub fn command(jsprint: &mut JSprint, _line: &str) {
    // Get current active sprint
    let sprint = jsprint.get_active_sprint().unwrap();

    println!("On Review in sprint {}", sprint.name);

    // Get issues
    let issues = jsprint.get_issues_on_review(&sprint);

    if issues.is_empty() {
        println!("No issues found for sprint {}", sprint.name);
        return;
    }

    let permalink_padding = issues.permalink_padding(&jsprint.jira);

    for (assignee, mut user_issues) in issues.group_by_assignee() {
        println!();
        println!("{}", assignee.bold());

        user_issues.sort_unstable_by(|a, b| a.updated().cmp(&b.updated()));

        for issue in user_issues {
            let permalink = issue.permalink(&jsprint.jira);
            let summary = issue.display_summary().bold();
            let days_on_review = issue
                .days_on_review()
                .map_or("x".to_owned(), |d| d.to_string());
            let review_level = match issue.review_level() {
                ReviewLevel::Low => "**".green(),
                ReviewLevel::Medium => "****".yellow(),
                ReviewLevel::High => "******".red().bold(),
                ReviewLevel::Unknown => "------".bold(),
            };

            println!(
                "{:review_padding$} ({:>2}d) - {:permalink_padding$} {}",
                review_level,
                days_on_review,
                permalink,
                summary,
                review_padding = REVIEW_PADDING,
                permalink_padding = permalink_padding,
            );
        }
    }
}
