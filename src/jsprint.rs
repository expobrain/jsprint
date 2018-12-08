use goji::*;
use std::io::{self, Write};

use crate::extend_issues::Issues;
use crate::settings::Settings;

#[derive(Debug)]
pub struct JSprint {
    pub settings: Settings,
    pub jira: Jira,
    board: Board,
    pub current_sprint: Option<Sprint>,
}

impl JSprint {
    pub fn new(settings: Settings) -> JSprint {
        print!("Connecting to Jira...");

        io::stdout().flush().unwrap();

        let jira = Jira::new(
            settings.jira_url.clone(),
            Credentials::Basic(
                settings.jira_username.clone(),
                settings.jira_password.clone(),
            ),
        )
        .unwrap();

        let board = jira
            .boards()
            .get(settings.jira_board_id.to_string())
            .unwrap();

        println!("done!");

        JSprint {
            jira,
            settings,
            board,
            current_sprint: None,
        }
    }

    pub fn get_sprints<'a>(&'a mut self, options: &'a SearchOptions) -> Option<SprintsIter> {
        self.jira.sprints().iter(&self.board, options).ok()
    }

    pub fn get_sprint(&mut self, sprint_id: u64) -> Option<Sprint> {
        self.jira.sprints().get(sprint_id).ok()
    }

    pub fn get_active_sprint(&mut self) -> Option<Sprint> {
        let options = SearchOptions::builder().state("active").build();

        self.get_sprints(&options).and_then(|r| r.take(1).next())
    }

    fn jql_filters(&self, sprint: &Sprint) -> Vec<String> {
        // Prepare JQL query
        let mut jql = vec![];

        jql.push(format!("project = '{}'", self.settings.jira_project));
        jql.push(format!("sprint = {}", sprint.id));

        if !self.settings.team_members.is_empty() {
            jql.push(format!(
                "(assignee IS NULL or assignee IN ({}))",
                self.settings.team_members.join(",")
            ));
        }

        if !self.settings.team_labels.is_empty() {
            jql.push(format!(
                "(labels IS NULL or labels IN ({}))",
                self.settings.team_labels.join(",")
            ));
        }

        jql
    }

    pub fn get_issues_on_review(&self, sprint: &Sprint) -> Issues {
        // Build filter
        let mut jql_filters = self.jql_filters(&sprint);
        jql_filters.push("(status = \"Resolved\" or status = \"On Review\")".to_string());

        // Build options
        let options = SearchOptionsBuilder::new()
            .jql(&jql_filters.join(" AND "))
            .validate_query(true)
            .build();

        // Return issues
        self.jira
            .issues()
            .iter(&self.board, &options)
            .ok()
            .map(|r| r.collect())
            .unwrap_or_default()
    }

    pub fn get_issues(&self, sprint: &Sprint) -> Issues {
        // Build options
        let jql = self.jql_filters(&sprint).join(" AND ");
        let options = SearchOptionsBuilder::new()
            .jql(&jql)
            .validate_query(true)
            .build();

        // Return issues
        self.jira
            .issues()
            .iter(&self.board, &options)
            .ok()
            .map(|r| r.collect())
            .unwrap_or_default()
    }
}
