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
use std::boxed::Box;

mod command;
mod commands;
mod commands_processor;
mod extend_issue;
mod extend_issues;
mod extend_sprints;
mod jsprint;
mod settings;

use crate::commands_processor::CommandProcessor;
use crate::jsprint::*;
use crate::settings::*;

fn main() {
    // Load settings
    let settings = Settings::new();

    // Connect to Jira
    let mut jsprint = JSprint::new(settings);

    jsprint.current_sprint = jsprint.get_active_sprint();

    // Register commands
    let mut processor = CommandProcessor::new();

    processor.register_command("sp", Box::new(commands::sprint::command));
    processor.register_command("sps", Box::new(commands::sprints::command));
    processor.register_command("rp", Box::new(commands::report::command));
    processor.register_command("rw", Box::new(commands::reviews::command));
    processor.register_command("bk", Box::new(commands::backlog::command));
    processor.register_command("lcount", Box::new(commands::labels_count::command));
    processor.register_command("u", Box::new(commands::use_sprint::command));
    processor.register_command("a", Box::new(commands::add_to_sprint::command));
    processor.register_command("as", Box::new(commands::assign::command));

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
                    let line_ref: &str = line.as_ref();

                    rl.add_history_entry(line_ref);
                    processor.process(&mut jsprint, line_ref);
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
