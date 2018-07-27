use serde_json;
use std::collections::BTreeMap;
use std::env;
use std::fs::File;
use std::io::{self, Write};

#[derive(Deserialize, Debug)]
pub struct Settings {
    pub jira_url: String,
    pub jira_username: String,
    pub jira_password: String,
    pub jira_project: String,
    pub jira_board_id: u32,
    #[serde(default)]
    pub team_members: Vec<String>,
    #[serde(default)]
    pub team_labels: Vec<String>,
    #[serde(default)]
    pub labels_mapping: BTreeMap<String, String>,
}

impl Settings {
    pub fn new() -> Settings {
        print!("Loading settings...");

        io::stdout().flush().unwrap();

        // Get full settings path
        let mut settings_path = env::current_dir().unwrap();
        settings_path.push("settings.json");

        // Read file content
        let f = File::open(settings_path).expect("file not found");

        // Deserialise settings
        let settings: Settings = serde_json::from_reader(f).unwrap();

        println!("done!");

        settings
    }
}
