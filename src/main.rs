use clap::Parser;
use config::AppConfig;
use std::error::Error;
use std::process;
use std::result::Result;

mod cli;
mod config;
mod github;
mod gitlab;
mod openai;

#[tokio::main]
async fn main() {
    if let Err(e) = execute().await {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

async fn execute() -> Result<(), Box<dyn Error>> {
    let cli = cli::Cli::parse();
    let mut config = config::load_config()?;

    match &cli.command {
        cli::Commands::Config { action } => match action {
            cli::ConfigAction::GitLabToken { token } => config.save_gitlab_token(token),
            cli::ConfigAction::GitHubToken { token } => config.save_github_token(token),
            cli::ConfigAction::OpenAiToken { token } => config.save_openai_token(token),
        },
        cli::Commands::Run { pr_link } => Ok(run(&config, pr_link).await?),
    }?;

    Ok(())
}

async fn run(config: &AppConfig, pr_link: &String) -> Result<(), Box<dyn Error>> {
    let combined_diffs: String = if pr_link.contains("github.com") {
        let github_token = config
            .get_github_token()
            .ok_or("GitHub token is not configured.")?;
        let (owner, repo, pr_number) = github::extract_info(pr_link);

        github::fetch_and_combine_diffs(&github_token, &owner, &repo, pr_number).await?
    } else if pr_link.contains("gitlab.com") {
        let gitlab_token = config
            .get_gitlab_token()
            .ok_or("GitLab token is not configured.")?;
        let (project_id, mr_id) = gitlab::extract_info(pr_link);

        gitlab::fetch_and_combine_diffs(gitlab_token, project_id, mr_id).await?
    } else {
        return Err("Unsupported PR link format.".into());
    };

    let prompt = add_code_reviewer_prompt(&combined_diffs);

    //println!("Prompt: {}", prompt);

    let openai_token = config
        .get_openai_token()
        .ok_or("Open AI token is not configured.")?;

    let response =
        openai::send_prompt_to_openai_code_reviwer(prompt.as_str(), openai_token).await?;
    println!("Response: {}", response);
    Ok(())
}

fn add_code_reviewer_prompt(combined_diffs: &String) -> String {
    format!(
        r#"
        Analyze the provided code changes in the Merge Request (MR) for potential issues and suggest improvements. Your review should cover:

        Syntax: Identify and correct syntax errors.
        Logic: Highlight logical errors or potential issues in the code flow.
        Efficiency: Suggest optimizations for better performance.
        Style: Ensure adherence to the company's coding standards and style guides.
        Security: Identify potential security vulnerabilities.
        Guidelines:
        
        Focus on changed lines: Lines starting with '+' or '-'.
        Ignore unchanged lines: Lines starting with ' '.
        Avoid repetition: Comment only once on identical issues.
        Format feedback with Markdown: Use bullet points and code snippets.
        Write 'EMPTY_CODE_REVIEW' if there are no issues in whole MR.
        Example output format:
        - **Issue:** Potential null pointer exception.
        **Suggestion:** Add a null check before accessing the object.
        **Code:** `if (object != null) {{ // use object }}`
        - **Issue:** Inefficient loop.
        **Suggestion:** Replace with a more efficient algorithm.
        **Code:** `for (int i = 0; i < list.size(); i++) {{ // optimized code }}`
        Here are the code changes:
    {}"#,
        combined_diffs
    )
}
