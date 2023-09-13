use rss::Channel;
use serde::Serialize;
use thiserror::Error;

type Result<T, E = RssError> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum RssError {
    #[error("http client error")]
    ClientError(#[from] reqwest::Error),

    #[error("rss error")]
    RssError(#[from] rss::Error),
}

pub fn get_posts<T>(url: T, count: Option<u64>) -> Result<Vec<Post>>
where
    T: reqwest::IntoUrl,
{
    let content = reqwest::blocking::get(url)?.bytes()?;
    let channel = Channel::read_from(&content[..])?;

    let mut posts: Vec<Post> = channel
        .items
        .into_iter()
        .map(|i| Post {
            published_at: i.pub_date.unwrap_or_default(),
            description: i.description.unwrap_or_default(),
            title: i.title.unwrap_or_default(),
            link: i.link.unwrap_or_default(),
        })
        .collect();

    if let Some(count) = count {
        posts.truncate(count as usize)
    }

    Ok(posts)
}

#[derive(Debug, Serialize)]
pub struct Post {
    pub published_at: String,
    pub description: String,
    pub title: String,
    pub link: String,
}
