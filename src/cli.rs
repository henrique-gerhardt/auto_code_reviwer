use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
    Run {
        #[arg(index = 1)]
        pr_link: String,
    },
}

#[derive(Subcommand)]
pub enum ConfigAction {
    GitLabToken { token: String },
    GitHubToken { token: String },
    OpenAiToken { token: String },
}