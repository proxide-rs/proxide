use graphql_client::{reqwest::post_graphql, GraphQLQuery};
use reqwest::{header::InvalidHeaderValue, Client};
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

#[derive(Debug)]
pub struct GithubClient {
    client: Client,
}

pub type Result<T, E = GithubClientError> = std::result::Result<T, E>;

impl GithubClient {
    pub fn new(api_token: &str) -> Result<Self> {
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

        Ok(Self { client })
    }

    pub async fn get_contributions(&self, username: String) -> Result<()> {
        let variables = contributions::Variables { username };

        let response =
            post_graphql::<Contributions, _>(&self.client, GITHUB_GRAPHQL_API_URL, variables)
                .await?;

        let date: contributions::ResponseData = response.data.ok_or(GithubClientError::NoData)?;

        todo!()
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
    schema_path = "schema/schema.graphql",
    query_path = "schema/queries/contributions.graphql",
    response_derives = "Debug"
)]
pub struct Contributions;
