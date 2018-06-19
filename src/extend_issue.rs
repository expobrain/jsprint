use chrono::prelude::*;
use colored::*;
use goji::Issue;

const NO_SUMMARY: &str = "<no summary>";
const NO_STATUS: &str = "<no status>";
const REVIEW_LOW_THRESHOLD: i64 = 1;
const REVIEW_HIGH_THRESHOLD: i64 = 3;

pub enum ReviewLevel {
    Low,
    Medium,
    High,
    Unknown,
}

pub trait IssueReviewLevel {
    fn days_on_review(&self) -> Option<i64>;
    fn review_level(&self) -> ReviewLevel;
}

pub trait IssueDisplayable {
    fn display_summary(&self) -> String;
    fn colored_status(&self) -> ColoredString;
}

impl IssueReviewLevel for Issue {
    fn days_on_review(&self) -> Option<i64> {
        self.resolution_date()
            .map(|v| v.parse::<DateTime<Utc>>().unwrap())
            .map(|v| Utc::now() - v)
            .map(|v| v.num_days())
    }

    fn review_level(&self) -> ReviewLevel {
        self.days_on_review()
            .map(|v| match v {
                v if v <= REVIEW_LOW_THRESHOLD => ReviewLevel::Low,
                v if v > REVIEW_LOW_THRESHOLD && v <= REVIEW_HIGH_THRESHOLD => ReviewLevel::Medium,
                _ => ReviewLevel::High,
            })
            .unwrap_or(ReviewLevel::Unknown)
    }
}

impl IssueDisplayable for Issue {
    fn display_summary(&self) -> String {
        self.summary()
            .map(|s| s)
            .unwrap_or_else(|| NO_SUMMARY.to_string())
    }

    fn colored_status(&self) -> ColoredString {
        self.status()
            .map(|s| s.name)
            .map(|name| match name.as_ref() {
                "Backlog" => name.blue(),
                "Open" | "In Progress" | "Reopened" => name.red(),
                "On Review" => name.yellow(),
                "Resolved" | "Closed" | "On Production" | "In Build - Ok" => name.green(),
                _ => name.normal(),
            })
            .unwrap_or_else(|| NO_STATUS.normal())
    }
}
