use async_trait::async_trait;
use futures::Future;

use crate::jav::ds::*;

pub async fn rvec<T, F>(fut: F) -> Result<(), Vec<T>>
where
    F: Future<Output = Vec<T>>,
{
    let r = fut.await;
    if r.len() > 0 {
        Err(r)
    } else {
        Ok(())
    }
}

pub async fn ropt<T, F>(fut: F) -> Result<(), T>
where
    F: Future<Output = Option<T>>,
{
    match fut.await {
        Some(res) => Err(res),
        None => Ok(()),
    }
}

#[async_trait]
pub trait SearchByCode {
    async fn search_by_code(code: &String) -> Option<AV>;
}

#[async_trait]
pub trait SearchByActress {
    async fn search_by_actress(actress: &String) -> Vec<AV>;
}

#[async_trait]
pub trait GetBrief {
    async fn get_brief(code: &String) -> Option<AV>;
}

#[async_trait]
pub trait GetAliases {
    async fn get_aliases(actress: &String) -> Vec<String>;
}

#[async_trait]
pub trait Translate2JP {
    async fn translate2jp(actress: &String) -> Option<String>;
}

#[async_trait]
pub trait GetMagnets {
    async fn get_magnets(code: &String) -> Vec<Magnet>;
}

#[async_trait]
pub trait GetNewlyReleased {
    async fn get_newly_released(page: u32) -> Vec<AV>;
}
