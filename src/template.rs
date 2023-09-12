use std::{collections::HashMap, io::Write};

use tera::{to_value, Context, Function, Tera, Value};
use thiserror::Error;

use crate::sources::github::{GithubClient, GithubClientError};

pub type Result<T, E = RendererError> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum RendererError {
    #[error("github client error")]
    GithubClientError(#[from] GithubClientError),

    #[error("template error")]
    TemplateError(#[from] tera::Error),
}

pub struct Renderer {
    context: Context,
    renderer: Tera,
}

impl Renderer {
    pub fn new(github_api_token: &str, github_username: String) -> Result<Self> {
        let github_client = GithubClient::new(github_api_token, github_username.clone())?;

        let mut renderer = Tera::default();

        renderer.register_function(
            "recentContributions",
            recent_contributions(github_client.clone()),
        );

        renderer.register_function(
            "recentPullRequests",
            recent_pull_requests(github_client.clone()),
        );

        renderer.register_function(
            "recentRepositories",
            recent_repositories(github_client.clone()),
        );

        let mut context = Context::new();
        context.insert("github_username", &github_username);

        Ok(Self { renderer, context })
    }

    pub fn render(&mut self, template: &str, to: impl Write) -> Result<()> {
        self.renderer.add_raw_template("template", template)?;
        Ok(self.renderer.render_to("template", &self.context, to)?)
    }
}

fn recent_contributions(github_client: GithubClient) -> impl Function {
    move |args: &HashMap<String, Value>| -> tera::Result<Value> {
        let count = match args.get("count") {
            Some(v) => match v.as_i64() {
                Some(v) => Some(v),
                None => None,
            },
            None => None,
        };

        let contributions = match github_client.get_contributions(count) {
            Ok(contributions) => contributions,
            Err(err) => return Err(err.to_string().into()),
        };

        Ok(to_value(contributions).unwrap())
    }
}

fn recent_pull_requests(github_client: GithubClient) -> impl Function {
    move |args: &HashMap<String, Value>| -> tera::Result<Value> {
        let count = match args.get("count") {
            Some(v) => match v.as_i64() {
                Some(v) => Some(v),
                None => None,
            },
            None => None,
        };

        let pull_requests = match github_client.get_pull_requests(count) {
            Ok(pull_requests) => pull_requests,
            Err(err) => return Err(err.to_string().into()),
        };

        Ok(to_value(pull_requests).unwrap())
    }
}

fn recent_repositories(github_client: GithubClient) -> impl Function {
    move |args: &HashMap<String, Value>| -> tera::Result<Value> {
        let count = match args.get("count") {
            Some(v) => match v.as_i64() {
                Some(v) => Some(v),
                None => None,
            },
            None => None,
        };

        let repositories = match github_client.get_repositories(count) {
            Ok(repositories) => repositories,
            Err(err) => return Err(err.to_string().into()),
        };

        Ok(to_value(repositories).unwrap())
    }
}
