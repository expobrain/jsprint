pub mod add_to_sprint;
pub mod assign;
pub mod backlog;
pub mod labels_count;
pub mod report;
pub mod reviews;
pub mod sprint;
pub mod sprints;
pub mod use_sprint;

fn get_issue_key_from_number(issue_str: &str) -> Result<String, &'static str> {
    match issue_str.parse::<u32>() {
        Ok(issue) => Ok(format!("BIDEV-{}", issue)),
        Err(_) => {
            println!(" {:#?}", issue_str);
            Err("Issue number is not a number")
        }
    }
}
