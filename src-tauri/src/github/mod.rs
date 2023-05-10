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
    // Set the repository name.

    // Create a `reqwest` client with a user agent.
    let client = reqwest::blocking::Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()
        .unwrap();

    // Switch repo to fetch from based on project
    let repo_name = match project {
        Project::FlightCore => FLIGHTCORE_REPO_NAME,
        Project::Northstar => NORTHSTAR_RELEASE_REPO_NAME,
    };

    // Fetch the list of tags for the repository as a `Vec<Tag>`.
    let tags_url = format!("https://api.github.com/repos/{}/tags", repo_name);
    let tags: Vec<Tag> = client.get(tags_url).send().unwrap().json().unwrap();

    // Map each `Tag` element to a `TagWrapper` element with the desired label and `Tag` value.
    let tag_wrappers: Vec<TagWrapper> = tags
        .into_iter()
        .map(|tag| TagWrapper {
            label: tag.name.clone(),
            value: tag,
        })
        .collect();

    Ok(tag_wrappers)
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
        repo, first_tag.name, second_tag.name
    );

    let comparison: Comparison = client.get(comparison_url).send().unwrap().json().unwrap();
    let commits = comparison.commits;

    // Display the list of commits.
    println!(
        "Commits between {} and {}:",
        first_tag.name, second_tag.name
    );

    // Iterate over all commits in the diff
    for commit in commits {
        println!(
            "  * {} : {}",
            commit.sha,
            commit.commit.message.split('\n').next().unwrap()
        );
        patch_notes.push(
            commit
                .commit
                .message
                .split('\n')
                .next()
                .unwrap()
                .to_string(),
        );
    }

    full_patch_notes += &generate_flightcore_release_notes(patch_notes);

    Ok(full_patch_notes.to_string())
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
                    "style" => "**Styles:**",
                    "refactor" => "**Code Refactoring:**",
                    "build" => "**Build:**",
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
    // Fetch the list of commits between the two tags.

    // Create a `reqwest` client with a user agent.
    let client = reqwest::blocking::Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()
        .unwrap();

    let repos = [
        "R2Northstar/Northstar",
        "R2Northstar/NorthstarLauncher",
        "R2Northstar/NorthstarMods",
    ];

    let mut full_patch_notes = "".to_string();
    let mut authors_set = std::collections::HashSet::new();

    for repo in repos {
        full_patch_notes += &format!("{}\n\n", repo);

        let mut patch_notes: Vec<String> = [].to_vec();
        println!("{}", repo);
        // let repo = "R2Northstar/NorthstarLauncher";
        let comparison_url = format!(
            "https://api.github.com/repos/{}/compare/{}...{}",
            repo, first_tag.name, second_tag.name
        );

        log::info!("Compare URL: {}", comparison_url.clone());
        let comparison: Comparison = client.get(&comparison_url).send().unwrap().json().unwrap();
        let commits = comparison.commits;

        // Display the list of commits.
        println!(
            "Commits between {} and {}:",
            first_tag.name, second_tag.name
        );

        //
        for commit in commits {
            println!(
                "  * {} : {}",
                commit.sha,
                turn_pr_number_into_link(commit.commit.message.split('\n').next().unwrap(), repo)
            );
            patch_notes.push(turn_pr_number_into_link(
                commit.commit.message.split('\n').next().unwrap(),
                repo,
            ));

            // Store authors in set
            if commit.author.is_some() {
                authors_set.insert(commit.author.unwrap().login);
            }
        }

        full_patch_notes += &patch_notes.join("\n");
        full_patch_notes += "\n\n\n";
    }

    // Convert the set to a sorted vector.
    let mut sorted_vec: Vec<String> = authors_set.into_iter().collect();
    sorted_vec.sort();

    // Define a string to prepend to each element.
    let prefix = "@";

    // Create a new list with the prefix prepended to each element.
    let prefixed_list: Vec<String> = sorted_vec.iter().map(|s| prefix.to_owned() + s).collect();

    full_patch_notes += "**Contributors:**\n";
    full_patch_notes += &prefixed_list.join(" ");

    Ok(full_patch_notes.to_string())
}

/// Takes the commit title and repo slug and formats it as
/// `[commit title(SHORTENED_REPO#NUMBER)](LINK)`
fn turn_pr_number_into_link(input: &str, repo: &str) -> String {
    // Extract `Mods/Launcher` from repo title
    let last_line = repo
        .split('/')
        .rev()
        .next()
        .unwrap()
        .trim_start_matches("Northstar");
    // Extract PR number
    let re = Regex::new(r"#(\d+)").unwrap();

    // Generate pull request link
    let pull_link = format!("https://github.com/{}/pull/", repo);
    re.replace_all(input, format!("[{}#$1]({}$1)", last_line, pull_link))
        .to_string()
}
