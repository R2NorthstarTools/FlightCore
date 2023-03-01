pub mod release_notes;

use app::constants::{APP_USER_AGENT, SECTION_ORDER};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, Debug, Clone, TS)]
#[ts(export)]
pub struct Tag {
    name: String,
}

#[derive(Debug, Deserialize)]
struct CommitInfo {
    sha: String,
    commit: Commit,
}

#[derive(Debug, Deserialize)]
struct Commit {
    message: String,
}

#[derive(Debug, Deserialize)]
struct Comparison {
    commits: Vec<CommitInfo>,
}

/// Get a list of tags on the FlightCore repo
#[tauri::command]
pub fn get_list_of_tags() -> Result<Vec<Tag>, String> {
    // Set the repository name.
    let repo = "R2NorthstarTools/FlightCore";

    // Create a `reqwest` client with a user agent.
    let client = reqwest::blocking::Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()
        .unwrap();

    // Fetch the list of tags for the repository.
    let tags_url = format!("https://api.github.com/repos/{}/tags", repo);
    let tags: Vec<Tag> = client.get(&tags_url).send().unwrap().json().unwrap();

    Ok(tags)
}

/// Use GitHub API to compare two tags of the same repo against each other and get the resulting changes
#[tauri::command]
pub fn compare_tags(first_tag: String, second_tag: String) -> Result<String, String> {
    // pub fn compare_tags(first_tag: Tag, second_tag: Tag) -> Result<(), String> {
    // TODO args should be `Tag` not `String`
    // Fetch the list of commits between the two tags.

    // Create a `reqwest` client with a user agent.
    let client = reqwest::blocking::Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()
        .unwrap();

    let repo = "R2NorthstarTools/FlightCore";

    let mut full_patch_notes = "".to_string();

    let mut patch_notes: Vec<String> = [].to_vec();
    println!("{}", repo);
    // let repo = "R2Northstar/NorthstarLauncher";
    let comparison_url = format!(
        "https://api.github.com/repos/{}/compare/{}...{}",
        // repo, first_tag.name, second_tag.name
        repo,
        first_tag,
        second_tag
    );

    dbg!(comparison_url.clone());
    let comparison: Comparison = client.get(&comparison_url).send().unwrap().json().unwrap();
    let commits = comparison.commits;
    dbg!();

    // Display the list of commits.
    println!(
        "Commits between {} and {}:",
        // first_tag.name, second_tag.name
        first_tag,
        second_tag
    );

    // Iterate over all commits in the diff
    for commit in commits {
        println!(
            "  * {} : {}",
            commit.sha,
            commit.commit.message.split('\n').next().unwrap()
        );
        patch_notes.push(format!(
            "{}",
            commit.commit.message.split('\n').next().unwrap()
        ));
    }

    full_patch_notes += &generate_flightcore_release_notes(patch_notes);

    Ok(full_patch_notes.to_string())
}

use std::collections::HashMap;

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
                    "style" => "**Styles:**",
                    "refactor" => "**Code Refactoring:**",
                    "build" => "**Build:**",
                    "test" => "**Tests:**",
                    "chore" => "**Chores:**",
                    _ => "**Other:**",
                };

                release_notes.push_str(&format!("{}\n", section_title));

                for commit_message in commit_list {
                    release_notes.push_str(&format!("- {}\n", commit_message));
                }

                release_notes.push_str("\n");
            }
        }
    }

    release_notes
}

/// Group semantic commit messages by type
/// Commmit messages that are not formatted accordingly are marked as "other"
fn group_commits_by_type(commits: Vec<String>) -> HashMap<String, Vec<String>> {
    let mut grouped_commits: HashMap<String, Vec<String>> = HashMap::new();
    let mut other_commits: Vec<String> = vec![];

    for commit in commits {
        let commit_parts: Vec<&str> = commit.splitn(2, ":").collect();
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
