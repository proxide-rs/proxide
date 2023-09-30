use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
pub struct Cli {
    /// Path tp template file
    #[arg(long)]
    pub(crate) template_file: PathBuf,

    /// GitHub API token
    #[arg(short, long, env("GITHUB_TOKEN"))]
    pub(crate) token: String,

    /// GitHub username
    #[arg(short, long, env("GITHUB_USERNAME"))]
    pub(crate) username: String,

    /// Path to output file, usually 'README.md'
    pub(crate) output_file: Option<PathBuf>,
}
