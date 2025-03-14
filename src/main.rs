use dotenv::dotenv;
use reqwest::header::{HeaderMap, AUTHORIZATION, USER_AGENT, ACCEPT};
use serde::{Deserialize, Serialize};
use std::env;

/// Represents a GitHub pull request.
#[derive(Debug, Serialize, Deserialize)]
struct PullRequest {}

/// Represents a GitHub issue with its number, title, and optional pull request reference.
#[derive(Debug, Serialize, Deserialize)]
struct Issue {
    number: usize,
    title: String,
    pull_request: Option<PullRequest>,
}

/// Extracts and constructs a new URL from the `link` header if it exists.
/// This function helps in paginating through GitHub API responses.
/// 
/// An `Option<String>` containing the new URL if available.
fn construct_new_url(headers: &HeaderMap) -> Option<String> {
    headers
        .get("link")
        .and_then(|value| value.to_str().ok())
        .map(|s| s.to_string())
}

/// Fetches open issues from a GitHub repository.

/// The function loads the GitHub personal access token from the `.env` file,
/// makes an authenticated request to the GitHub API, and filters out pull requests.

/// A vector of `Issue` structs representing open issues in the repository.
async fn get_issues() -> Vec<Issue> {
    // Load environment variables from .env file
    dotenv().ok();

    // Retrieve GitHub token from environment variables
    let token = env::var("GITHUB_PAT").expect("Expected GITHUB_PAT in env file");

    // Construct API request URL
    let url = format!(
        "https://api.github.com/repos/{owner}/{repo}/issues",
        owner = "freeCodeCamp",
        repo = "freeCodeCamp"
    );

    // Initialize the HTTP client
    let client = reqwest::Client::new();

    // Send the GET request with authentication headers
    let response = client
        .get(&url)
        .header(AUTHORIZATION, format!("Bearer {}", token))
        .header(USER_AGENT, "rust web-api")
        .header(ACCEPT, "application/vnd.github+json")
        .send()
        .await;

    // Handle response errors
    let response = match response {
        Ok(res) if res.status().is_success() => res,
        _ => return Vec::new(),
    };

    // Attempt to extract pagination link (currently unused)
    let _new_url = construct_new_url(response.headers());

    // Deserialize JSON response into a vector of issues
    let issues = response
        .json::<Vec<Issue>>()
        .await
        .expect("Something went wrong while parsing")
        .into_iter()
        .filter(|issue| issue.pull_request.is_none()) // Exclude pull requests
        .collect::<Vec<Issue>>();

    issues
}

/// Entry point of the application.
#[tokio::main]
async fn main() {
    // Load environment variables
    dotenv().ok();

    // Fetch issues asynchronously
    let issues = get_issues().await;

    // Print retrieved issues
    println!("{:?}", issues);
}