pub mod release_notes;

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
    author: CommitAuthor,
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

#[tauri::command]
pub fn get_list_of_tags() -> Result<Vec<Tag>, String> {
    // Set the repository name.
    let repo = "R2NorthstarTools/FlightCore";

    // Create a `reqwest` client with a user agent.
    let client = reqwest::blocking::Client::builder()
        .user_agent("my-awesome-app")
        .build()
        .unwrap();

    // Fetch the list of tags for the repository.
    let tags_url = format!("https://api.github.com/repos/{}/tags", repo);
    let tags: Vec<Tag> = client.get(&tags_url).send().unwrap().json().unwrap();

    Ok(tags)
}

#[tauri::command]
pub fn compare_tags(first_tag: String, second_tag: String) -> Result<String, String> {
    // pub fn compare_tags(first_tag: Tag, second_tag: Tag) -> Result<(), String> {
    // TODO args should be `Tag` not `String`
    // Fetch the list of commits between the two tags.

    // Create a `reqwest` client with a user agent.
    let client = reqwest::blocking::Client::builder()
        .user_agent("my-awesome-app")
        .build()
        .unwrap();

    let repo = "R2NorthstarTools/FlightCore";

    let mut full_patch_notes = "".to_string();
    let mut authors_set = std::collections::HashSet::new();

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

    //
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

        // Store authors in set
        authors_set.insert(commit.author.login);
    }

    full_patch_notes += &generate_flightcore_release_notes(patch_notes);

    // Convert the set to a sorted vector.
    let mut sorted_vec: Vec<String> = authors_set.into_iter().collect();
    sorted_vec.sort();

    // Define a string to prepend to each element.
    let prefix = "@";

    // Create a new list with the prefix prepended to each element.
    let prefixed_list: Vec<String> = sorted_vec.iter().map(|s| prefix.to_owned() + s).collect();

    full_patch_notes += &"\n\n**Contributors:**\n";
    full_patch_notes += &prefixed_list.join(" ");

    Ok(full_patch_notes.to_string())
}

use std::collections::HashMap;

fn generate_flightcore_release_notes(commits: Vec<String>) -> String {
    let grouped_commits = group_commits_by_type(commits);
    let mut release_notes = String::new();

    // Order in which the sections should be displayed
    let section_order = vec![
        "feat", "fix", "docs", "style", "refactor", "test", "chore", "other",
    ];

    // Go over commit types and generate notes
    for commit_type in section_order {
        if let Some(commit_list) = grouped_commits.get(commit_type) {
            if !commit_list.is_empty() {
                let section_title = match commit_type {
                    "feat" => "**Features:**",
                    "fix" => "**Bug Fixes:**",
                    "docs" => "**Documentation:**",
                    "style" => "**Styles:**",
                    "refactor" => "**Code Refactoring:**",
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

    for commit in commits {
        let commit_parts: Vec<&str> = commit.splitn(2, ":").collect();
        if commit_parts.len() == 2 {
            let commit_type = commit_parts[0].trim().to_lowercase();
            let commit_message = commit_parts[1].trim().to_string();
            let commit_list = grouped_commits.entry(commit_type).or_insert(vec![]);
            commit_list.push(commit_message);
        } else {
            let commit_list = grouped_commits.entry("other".to_string()).or_insert(vec![]);
            commit_list.push(commit.to_string());
        }
    }

    grouped_commits
}
