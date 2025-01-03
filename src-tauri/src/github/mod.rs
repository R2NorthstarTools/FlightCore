pub mod pull_requests;
pub mod release_notes;

use crate::constants::{
    APP_USER_AGENT, FLIGHTCORE_REPO_NAME, NORTHSTAR_RELEASE_REPO_NAME, SECTION_ORDER,
};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use ts_rs::TS;

#[derive(Serialize, Deserialize, Debug, Clone, TS)]
#[ts(export)]
pub struct Tag {
    name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, TS)]
#[ts(export)]
pub enum Project {
    FlightCore,
    Northstar,
}

/// Wrapper type needed for frontend
#[derive(Serialize, Deserialize, Debug, Clone, TS)]
#[ts(export)]
pub struct TagWrapper {
    label: String,
    value: Tag,
}

#[derive(Debug, Deserialize)]
pub struct CommitInfo {
    pub sha: String,
    commit: Commit,
    author: Option<CommitAuthor>,
}

#[derive(Debug, Deserialize)]
struct Commit {
    message: String,
}

#[derive(Debug, Deserialize)]
struct CommitAuthor {
    login: String,
}

#[derive(Debug, Deserialize)]
struct Comparison {
    commits: Vec<CommitInfo>,
}

/// Get a list of tags on the FlightCore repo
#[tauri::command]
pub fn get_list_of_tags(project: Project) -> Result<Vec<TagWrapper>, String> {
    todo!()
}

/// Use GitHub API to compare two tags of the same repo against each other and get the resulting changes
#[tauri::command]
pub fn compare_tags(project: Project, first_tag: Tag, second_tag: Tag) -> Result<String, String> {
    match project {
        Project::FlightCore => compare_tags_flightcore(first_tag, second_tag),
        Project::Northstar => compare_tags_northstar(first_tag, second_tag),
    }
}

pub fn compare_tags_flightcore(first_tag: Tag, second_tag: Tag) -> Result<String, String> {
    todo!()
}

/// Generate release notes in the format used for FlightCore
fn generate_flightcore_release_notes(commits: Vec<String>) -> String {
    let grouped_commits = group_commits_by_type(commits);
    let mut release_notes = String::new();

    // Go over commit types and generate notes
    for commit_type in SECTION_ORDER {
        if let Some(commit_list) = grouped_commits.get(commit_type) {
            if !commit_list.is_empty() {
                let section_title = match commit_type {
                    "feat" => "**Features:**",
                    "fix" => "**Bug Fixes:**",
                    "docs" => "**Documentation:**",
                    "style" => "**Code style changes:**",
                    "refactor" => "**Code Refactoring:**",
                    "build" => "**Build:**",
                    "ci" => "**Continuous integration changes:**",
                    "test" => "**Tests:**",
                    "chore" => "**Chores:**",
                    "i18n" => "**Translations:**",
                    _ => "**Other:**",
                };

                release_notes.push_str(&format!("{}\n", section_title));

                for commit_message in commit_list {
                    release_notes.push_str(&format!("- {}\n", commit_message));
                }

                release_notes.push('\n');
            }
        }
    }

    let release_notes = release_notes.trim_end_matches('\n').to_string();
    release_notes
}

/// Group semantic commit messages by type
/// Commmit messages that are not formatted accordingly are marked as "other"
fn group_commits_by_type(commits: Vec<String>) -> HashMap<String, Vec<String>> {
    let mut grouped_commits: HashMap<String, Vec<String>> = HashMap::new();
    let mut other_commits: Vec<String> = vec![];

    for commit in commits {
        let commit_parts: Vec<&str> = commit.splitn(2, ':').collect();
        if commit_parts.len() == 2 {
            let commit_type = commit_parts[0].to_lowercase();
            let commit_description = commit_parts[1].trim().to_string();

            // Check if known commit type
            if SECTION_ORDER.contains(&commit_type.as_str()) {
                let commit_list = grouped_commits.entry(commit_type.to_string()).or_default();
                commit_list.push(commit_description);
            } else {
                // otherwise add to list of "other"
                other_commits.push(commit.to_string());
            }
        } else {
            other_commits.push(commit.to_string());
        }
    }
    grouped_commits.insert("other".to_string(), other_commits);

    grouped_commits
}

/// Compares two tags on Northstar repo and generates release notes over the diff in tags
/// over the 3 major repos (Northstar, NorthstarLauncher, NorthstarMods)
pub fn compare_tags_northstar(first_tag: Tag, second_tag: Tag) -> Result<String, String> {
    todo!()
}

/// Takes the commit title and repo slug and formats it as
/// `[commit title(SHORTENED_REPO#NUMBER)](LINK)`
fn turn_pr_number_into_link(input: &str, repo: &str) -> String {
    // Extract `Mods/Launcher` from repo title
    let last_line = repo
        .split('/')
        .next_back()
        .unwrap()
        .trim_start_matches("Northstar");
    // Extract PR number
    let re = Regex::new(r"#(\d+)").unwrap();

    // Generate pull request link
    let pull_link = format!("https://github.com/{}/pull/", repo);
    re.replace_all(input, format!("[{}#$1]({}$1)", last_line, pull_link))
        .to_string()
}
