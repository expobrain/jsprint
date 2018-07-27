extern crate chrono;
extern crate colored;
extern crate goji;
extern crate rustyline;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use rustyline::error::ReadlineError;
use rustyline::Editor;

mod command;
mod commands;
mod commands_processor;
mod extend_issue;
mod extend_issues;
mod extend_sprints;
mod jsprint;
mod settings;

use commands_processor::CommandProcessor;
use jsprint::*;
use settings::*;

fn main() {
    // Load settings
    let settings = Settings::new();

    // Connect to Jira
    let mut jsprint = JSprint::new(settings);

    jsprint.current_sprint = jsprint.get_active_sprint();

    // Register commands
    let mut processor = CommandProcessor::new();

    processor.register_command("sp", commands::sprint::get_command());
    processor.register_command("sps", commands::sprints::get_command());
    processor.register_command("rp", commands::report::get_command());
    processor.register_command("rw", commands::reviews::get_command());
    processor.register_command("bk", commands::backlog::get_command());
    processor.register_command("lcount", commands::labels_count::get_command());

    // Start shell
    // `()` can be used when no completer is required
    let mut rl = Editor::<()>::new();

    loop {
        let current_sprint_name = jsprint
            .current_sprint
            .as_ref()
            .map(|s| s.name.clone())
            .unwrap_or_else(|| "<none>".to_string());

        let readline = rl.readline(format!("JSprint [{}] >>> ", current_sprint_name).as_ref());

        match readline {
            Ok(line) => match line.as_str() {
                "q" | "quit" => break,
                _ => {
                    rl.add_history_entry(line.as_ref());
                    processor.process(&mut jsprint, &line);
                }
            },
            Err(ReadlineError::Interrupted) => {
                break;
            }
            Err(ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}
