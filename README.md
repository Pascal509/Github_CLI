# Github_CLI
Fetching GitHub issues using reqwest &amp; dotenv!




# GitHub Issues CLI in Rust

## Overview
This is a Rust-based CLI tool that interacts with the GitHub API to fetch open issues from a given repository. It utilizes `reqwest` for HTTP requests, `serde` for JSON parsing, and `dotenv` for environment variable management.

## Features
- Fetches open issues from a specified GitHub repository.
- Uses authentication via a GitHub Personal Access Token (PAT) stored in a `.env` file.
- Filters out pull requests from the fetched issues.
- Parses and displays issues in the console.

## Technologies Used
- Rust 🦀
- `reqwest` - HTTP client for making API requests
- `serde` - Serialization and deserialization of JSON data
- `dotenv` - Loading environment variables from a `.env` file
- `tokio` - Asynchronous runtime for Rust

## Prerequisites
Before running the project, ensure you have:
- Rust installed (https://www.rust-lang.org/tools/install)
- A GitHub Personal Access Token (PAT) with `repo` scope enabled.
- A `.env` file containing `GITHUB_PAT=<your_token>`

## Installation
1. Clone this repository:
   ```sh
   git clone <repository-url>
   cd <repository-folder>
   ```
2. Install dependencies:
   ```sh
   cargo build
   ```
3. Create a `.env` file in the root directory:
   ```sh
   echo "GITHUB_PAT=your_personal_access_token" > .env
   ```
4. Run the CLI:
   ```sh
   cargo run
   ```

## Code Breakdown

### Structs
```rust
#[derive(Debug, Serialize, Deserialize)]
struct PullRequest {}

#[derive(Debug, Serialize, Deserialize)]
struct Issue {
    number: usize,
    title: String,
    pull_request: Option<PullRequest>,
}
```
- `PullRequest`: Represents a pull request.
- `Issue`: Represents an issue, with an optional `pull_request` field to differentiate between issues and PRs.

### Constructing the API Request
```rust
async fn get_issues() -> Vec<Issue> {
    dotenv().ok();
    let token = env::var("GITHUB_PAT").expect("Expected GITHUB_PAT in env file");
    let url = format!(
        "https://api.github.com/repos/{owner}/{repo}/issues",
        owner = "freeCodeCamp",
        repo = "freeCodeCamp"
    );
    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header(AUTHORIZATION, format!("Bearer {}", token))
        .header(USER_AGENT, "rust web-api")
        .header(ACCEPT, "application/vnd.github+json")
        .send()
        .await;
```
- Loads the `.env` file.
- Retrieves the `GITHUB_PAT` token.
- Constructs the GitHub API request URL.
- Sends a GET request with necessary headers.

### Filtering and Parsing Issues
```rust
let issues = response
    .json::<Vec<Issue>>()
    .await
    .expect("Something went wrong while parsing")
    .into_iter()
    .filter(|issue| issue.pull_request.is_none())
    .collect::<Vec<Issue>>();
```
- Parses JSON response into `Vec<Issue>`.
- Filters out pull requests.

### Running the CLI
```rust
#[tokio::main]
async fn main() {
    dotenv().ok();
    let issues = get_issues().await;
    println!("{:?}", issues);
}
```
- Loads `.env`.
- Fetches and prints issues.

## Future Improvements
- Add pagination support to fetch more issues.
- Allow users to specify a repository dynamically.
- Implement error handling and logging.
- Format output in a more readable way (e.g., JSON or tabular format).

## License
This project is licensed under the MIT License.

