use graphql_client::{reqwest::post_graphql_blocking, GraphQLQuery};
use reqwest::{blocking::Client, header::InvalidHeaderValue};
use serde::Serialize;
use thiserror::Error;

const GITHUB_GRAPHQL_API_URL: &str = "https://api.github.com/graphql";

#[allow(clippy::upper_case_acronyms)]
type URI = String;
type DateTime = chrono::DateTime<chrono::Utc>;

#[derive(Debug, Error)]
pub enum GithubClientError {
    #[error("invalid authorization header value; is your github token correct?")]
    InvalidAuthorizationValue(#[from] InvalidHeaderValue),

    #[error("http client error")]
    ClientError(#[from] reqwest::Error),

    #[error("graphql query didn't return any data")]
    NoData,
}

#[derive(Debug, Clone)]
pub struct GithubClient {
    username: String,
    client: Client,
}

pub type Result<T, E = GithubClientError> = std::result::Result<T, E>;

impl GithubClient {
    pub fn new(api_token: &str, username: String) -> Result<Self> {
        let client = Client::builder()
            .user_agent("proxide")
            .default_headers(
                std::iter::once((
                    reqwest::header::AUTHORIZATION,
                    reqwest::header::HeaderValue::from_str(&format!("Bearer {}", api_token))?,
                ))
                .collect(),
            )
            .build()?;

        Ok(Self { client, username })
    }

    pub fn get_contributions(&self) -> Result<Vec<Contribution>> {
        let variables = contributions::Variables {
            username: self.username.clone(),
        };

        let response = post_graphql_blocking::<Contributions, _>(
            &self.client,
            GITHUB_GRAPHQL_API_URL,
            variables,
        )?;

        let data: contributions::ResponseData = response.data.ok_or(GithubClientError::NoData)?;

        let contributions = data
            .user
            .unwrap()
            .contributions_collection
            .commit_contributions_by_repository
            .into_iter()
            .map(|c| Contribution {
                repo_desc: c.repository.description.unwrap_or_default(),
                repo_is_private: c.repository.is_private,
                repo_name: c.repository.name_with_owner,
                repo_url: c.repository.url,
            })
            .collect();

        Ok(contributions)
    }

    pub fn get_pull_requests(&self) -> Result<()> {
        todo!()
    }

    pub fn get_repositories(&self) -> Result<()> {
        todo!()
    }
}

#[derive(Debug, GraphQLQuery)]
#[graphql(
    query_path = "schema/queries/contributions.graphql",
    schema_path = "schema/schema.graphql",
    response_derives = "Debug"
)]
pub struct Contributions;

#[derive(Debug, Serialize)]
pub struct Contribution {
    pub repo_is_private: bool,
    pub repo_name: String,
    pub repo_desc: String,
    pub repo_url: String,
}
