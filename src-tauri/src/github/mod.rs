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
    let repo = "R2Northstar/NorthstarLauncher";

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
                turn_pr_number_into_link(commit.commit.message.split('\n').next().unwrap(), repo)
            );
            patch_notes.push(format!(
                "{}",
                turn_pr_number_into_link(commit.commit.message.split('\n').next().unwrap(), repo)
            ));

            // Store authors in set
            authors_set.insert(commit.author.login);
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

    full_patch_notes += &"**Contributors:**\n";
    full_patch_notes += &prefixed_list.join(" ");

    Ok(full_patch_notes.to_string())
}

use regex::Regex;

fn turn_pr_number_into_link(input: &str, repo: &str) -> String {
    let last_line = repo
        .split('/')
        .rev()
        .next()
        .unwrap()
        .trim_start_matches("Northstar");
    let re = Regex::new(r"#(\d+)").unwrap();
    let pull_link = format!("https://github.com/{}/pull/", repo);
    re.replace_all(input, format!("[{}#$1]({}$1)", last_line, pull_link))
        .to_string()
}
