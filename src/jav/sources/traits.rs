use async_trait::async_trait;
use reqwest;

use crate::jav::ds::AV;

#[async_trait]
pub trait SearchByCode {
    async fn search_by_code(code: String) -> Result<Option<AV>, reqwest::Error>;
}

#[async_trait]
pub trait SearchByActress {
    async fn search_by_actress(actress: String) -> Result<Vec<AV>, reqwest::Error>;
}

#[async_trait]
pub trait GetBrief {
    async fn get_brief(code: String) -> Result<Option<AV>, reqwest::Error>;
}

#[async_trait]
pub trait GetAliases {
    async fn get_aliases(name: String) -> Result<Vec<String>, reqwest::Error>;
}

#[async_trait]
pub trait Translate2JP {
    async fn translate2jp(name: String) -> Result<Option<String>, reqwest::Error>;
}
