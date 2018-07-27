use command::Command;
use jsprint::JSprint;

fn get_issue_key_from_number(issue_str: &str) -> Result<String, &'static str> {
    match issue_str.parse::<u32>() {
        Ok(issue) => Ok(format!("BIDEV-{}", issue)),
        Err(_) => Err("Issue number is not a number"),
    }
}

fn exec(jsprint: &mut JSprint, line: &str) {
    // Get issue numbers
    let issues = line
        .split_whitespace()
        .skip(1)
        .map(|s| get_issue_key_from_number(s).unwrap())
        .collect::<Vec<String>>();

    if issues.is_empty() {
        println!("At least one issue number is needed");
        return;
    }

    jsprint.jira.backlog().put(issues).unwrap();
}

pub fn get_command() -> Command {
    Command {
        exec: Box::new(exec),
    }
}
