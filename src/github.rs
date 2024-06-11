use reqwest::Error;
use serde::Deserialize;


#[derive(Deserialize, Debug)]
struct GitHubDiff {
    filename: String,
    patch: String,
}

pub fn extract_info(pr_link: &str) -> (String, String, u64) {
  // Example link: https://github.com/owner/repo/pull/1
  let parts: Vec<&str> = pr_link.split('/').collect();
  let owner = parts[3].to_string();
  let repo = parts[4].to_string();
  let pr_number = parts[6].parse().unwrap();
  (owner, repo, pr_number)
}

pub async fn fetch_and_combine_diffs(token: &str, owner: &str, repo: &str, pr_number: u64) -> Result<String, Error> {
  let url = format!("https://api.github.com/repos/{}/{}/pulls/{}/files", owner, repo, pr_number);
  let client = reqwest::Client::new();
  let response = client
      .get(&url)
      .header("Authorization", format!("token {}", token))
      .header("User-Agent", "Rust")
      .send()
      .await?
      .json::<Vec<GitHubDiff>>()
      .await?;
  
  let combined_diffs = response.into_iter()
      .map(|diff| format!("File: {}\nDiff:\n{}\n", diff.filename, diff.patch))
      .collect::<Vec<String>>()
      .join("\n");
  
  Ok(combined_diffs)
}