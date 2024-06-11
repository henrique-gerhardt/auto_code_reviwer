use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Diff {
    old_path: String,
    new_path: String,
    diff: String,
}

pub fn extract_info(pr_link: &str) -> (String, u64) {
    // Example link: https://gitlab.com/group/project/-/merge_requests/1
    let parts: Vec<&str> = pr_link.split('/').collect();
    let mr_id = parts.last().unwrap().parse().unwrap();
    let project_id = url::form_urlencoded::byte_serialize(
        &parts[(parts.len() - 5)..(parts.len() - 3)]
            .join("/")
            .as_bytes(),
    )
    .collect();
    (project_id, mr_id)
}

pub async fn fetch_and_combine_diffs(
    token: &str,
    project_id: String,
    mr_id: u64,
) -> Result<String, Error> {
    let url = format!(
        "https://gitlab.com/api/v4/projects/{}/merge_requests/{}/diffs",
        project_id, mr_id
    );
    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("PRIVATE-TOKEN", token)
        .send()
        .await?;

    if !response.status().is_success() {
        let body = response.text().await?;
        panic!("Request failed with response body:\n{}", body);
    }

    let response_json = response.json::<serde_json::Value>().await?;

    let diffs = response_json
        .as_array()
        .unwrap()
        .iter()
        .map(|change| Diff {
            old_path: change["old_path"].as_str().unwrap().to_string(),
            new_path: change["new_path"].as_str().unwrap().to_string(),
            diff: change["diff"].as_str().unwrap().to_string(),
        })
        .collect::<Vec<Diff>>();

    let combined_diffs = diffs
        .into_iter()
        .map(|diff| {
            format!(
                "File: {}\nOld Path: {}\nDiff:\n{}\n",
                diff.new_path, diff.old_path, diff.diff
            )
        })
        .collect::<Vec<String>>()
        .join("\n");

    Ok(combined_diffs)
}
