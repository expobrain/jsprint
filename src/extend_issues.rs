use goji::{Issue, Jira};
use std::collections::BTreeMap;

const UNASSIGNED: &str = "<unassigned>";

pub type Issues = Vec<Issue>;

pub trait IssuesPadding {
    fn permalink_padding(&self, jira: &Jira) -> usize;
    fn status_padding(&self) -> usize;
    fn key_padding(&self) -> usize;
}

pub trait IssuesGrouping {
    fn group_by_assignee(&self) -> BTreeMap<String, Vec<&Issue>>;
}

impl IssuesPadding for Issues {
    fn permalink_padding(&self, jira: &Jira) -> usize {
        self.iter()
            .map(|i| i.permalink(&jira).len())
            .max()
            .unwrap_or(0)
    }

    fn status_padding(&self) -> usize {
        self.iter()
            .map(|i| i.status().map(|s| s.name.len()).unwrap_or(0))
            .max()
            .unwrap()
    }

    fn key_padding(&self) -> usize {
        self.iter().map(|i| i.key.len()).max().unwrap()
    }
}

impl IssuesGrouping for Issues {
    fn group_by_assignee(&self) -> BTreeMap<String, Vec<&Issue>> {
        // Good balance between memory usage and allocations
        let capacity = self.len() / 2;

        self.iter().fold(BTreeMap::new(), |mut acc, issue| {
            let assignee = issue
                .assignee()
                .map_or(UNASSIGNED.to_string(), |a| a.display_name);

            acc.entry(assignee)
                .or_insert_with(|| Vec::with_capacity(capacity))
                .push(issue);

            acc
        })
    }
}
