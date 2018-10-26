use command::CommandFn;
use jsprint::JSprint;
use std::collections::BTreeMap;

pub struct CommandProcessor {
    commands: BTreeMap<String, CommandFn>,
}

impl CommandProcessor {
    pub fn new() -> Self {
        CommandProcessor {
            commands: BTreeMap::new(),
        }
    }

    pub fn register_command(&mut self, cmd: &str, command: CommandFn) {
        self.commands.insert(cmd.to_string(), command);
    }

    pub fn process(&self, jsprint: &mut JSprint, line: &str) {
        let cmd_name = line.split_whitespace().take(1).next();

        match cmd_name {
            None => {}
            Some("h") | Some("help") => self.help(),
            Some(cmd_name) => {
                if let Some(cmd) = self.commands.get(cmd_name) {
                    cmd(jsprint, line)
                } else {
                    println!("Unkwon command {}", cmd_name)
                }
            }
        };
    }

    fn help(&self) {
        for k in self.commands.keys() {
            println!("{}", k);
        }
    }
}
