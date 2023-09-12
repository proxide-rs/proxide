use std::fs;

use clap::Parser;

use crate::{cli::Cli, template::Renderer};

mod cli;
mod sources;
mod template;

fn main() {
    let cli = Cli::parse();

    let template = fs::read_to_string(cli.template_file).unwrap();
    let output_file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(cli.output_file)
        .unwrap();

    let mut renderer = Renderer::new(&cli.token, cli.username).unwrap();
    renderer.render(&template, output_file).unwrap();
}
