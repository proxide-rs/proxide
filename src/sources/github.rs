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

    pub fn get_username(&self) -> Result<String> {
        let data = self.make_request::<Viewer>(viewer::Variables {})?;
        Ok(data.viewer.login)
    }

    pub fn get_contributions(
        &self,
        count: Option<i64>,
        username: String,
    ) -> Result<Vec<Contribution>> {
        let variables = contributions::Variables { username };

        let data = self.make_request::<Contributions>(variables)?;

        let mut contributions: Vec<Contribution> = data
            .user
            .unwrap()
            .contributions_collection
            .commit_contributions_by_repository
            .into_iter()
            .filter(|c| !c.repository.is_private)
            .map(|c| {
                let edge = c.contributions.edges.unwrap()[0].clone();

                Contribution {
                    repo_desc: c.repository.description.unwrap_or_default(),
                    occurred_at: edge.unwrap().node.unwrap().occurred_at,
                    repo_is_private: c.repository.is_private,
                    repo_name: c.repository.name_with_owner,
                    repo_url: c.repository.url,
                }
            })
            .collect();

        // Sadly this is not chainable
        contributions.sort();
        contributions.reverse();
        contributions.truncate(count.unwrap_or(5) as usize);

        Ok(contributions)
    }

    pub fn get_pull_requests(
        &self,
        count: Option<i64>,
        username: String,
    ) -> Result<Vec<PullRequest>> {
        let variables = pull_requests::Variables {
            count: count.or(pull_requests::Variables::default_count()),
            username,
        };

        let data = self.make_request::<PullRequests>(variables)?;

        let pull_requests = data
            .user
            .unwrap()
            .pull_requests
            .edges
            .unwrap()
            .into_iter()
            .flatten()
            .filter_map(|p| {
                let pr = p.node.unwrap();

                if pr.repository.is_private {
                    return None;
                }

                Some(PullRequest {
                    created_at: pr.created_at,
                    title: pr.title,
                    url: pr.url,
                    repo: Repository {
                        desc: pr.repository.description.unwrap_or_default(),
                        name: pr.repository.name_with_owner,
                        url: pr.repository.url,
                    },
                })
            })
            .collect();

        Ok(pull_requests)
    }

    pub fn get_repositories(
        &self,
        count: Option<i64>,
        username: String,
    ) -> Result<Vec<Repository>> {
        let variables = repositories::Variables {
            count: count.or(pull_requests::Variables::default_count()),
            username,
        };

        let data = self.make_request::<Repositories>(variables)?;

        let repositories: Vec<Repository> = data
            .user
            .unwrap()
            .repositories
            .edges
            .unwrap()
            .into_iter()
            .flatten()
            .filter_map(|r| {
                let repo = r.node.unwrap();

                if repo.is_private {
                    return None;
                }

                Some(Repository {
                    desc: repo.description.unwrap_or_default(),
                    name: repo.name_with_owner,
                    url: repo.url,
                })
            })
            .collect();

        Ok(repositories)
    }

    fn make_request<Q>(&self, variables: Q::Variables) -> Result<Q::ResponseData>
    where
        Q: GraphQLQuery,
    {
        let response =
            post_graphql_blocking::<Q, _>(&self.client, GITHUB_GRAPHQL_API_URL, variables)?;

        response.data.ok_or(GithubClientError::NoData)
    }
}

#[derive(Debug, GraphQLQuery)]
#[graphql(
    query_path = "schema/queries/viewer.graphql",
    schema_path = "schema/schema.graphql",
    response_derives = "Debug, Clone"
)]
struct Viewer;

#[derive(Debug, GraphQLQuery)]
#[graphql(
    query_path = "schema/queries/contributions.graphql",
    schema_path = "schema/schema.graphql",
    response_derives = "Debug, Clone"
)]
struct Contributions;

#[derive(Debug, Serialize, PartialEq, Eq)]
pub struct Contribution {
    pub occurred_at: DateTime,
    pub repo_is_private: bool,
    pub repo_name: String,
    pub repo_desc: String,
    pub repo_url: String,
}

impl PartialOrd for Contribution {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.occurred_at.partial_cmp(&other.occurred_at)
    }
}

impl Ord for Contribution {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.occurred_at.cmp(&other.occurred_at)
    }
}

#[derive(Debug, GraphQLQuery)]
#[graphql(
    query_path = "schema/queries/pull_requests.graphql",
    schema_path = "schema/schema.graphql",
    variables_derives = "Default",
    response_derives = "Debug"
)]
struct PullRequests;

#[derive(Debug, Serialize)]
pub struct PullRequest {
    pub created_at: DateTime,
    pub repo: Repository,
    pub title: String,
    pub url: String,
}

#[derive(Debug, GraphQLQuery)]
#[graphql(
    query_path = "schema/queries/repositories.graphql",
    schema_path = "schema/schema.graphql",
    variables_derives = "Default",
    response_derives = "Debug"
)]
struct Repositories;

#[derive(Debug, Serialize)]
pub struct Repository {
    pub name: String,
    pub desc: String,
    pub url: String,
}
