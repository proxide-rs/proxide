use std::{fs, io};

use clap::Parser;

use crate::{cli::Cli, sources::github::GithubClient, template::Renderer};

mod cli;
mod sources;
mod template;

fn main() {
    let cli = Cli::parse();

    let template = fs::read_to_string(cli.template_file).unwrap();
    let client = GithubClient::new(&cli.token).unwrap();
    let username = client.get_username().unwrap();

    let mut renderer = Renderer::new(&client, username).unwrap();

    match cli.output_file {
        Some(path) => {
            let file = fs::OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(path)
                .unwrap();

            renderer.render(&template, file).unwrap()
        }
        None => renderer.render(&template, io::stdout()).unwrap(),
    }
}
