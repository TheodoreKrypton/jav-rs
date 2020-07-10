use crate::jav::sources::common::*;
use crate::jav::sources::traits::GetAliases;
use reqwest;

pub struct AVHelpMemoWiki;

#[async_trait]
impl GetAliases for AVHelpMemoWiki {
    async fn get_aliases(actress: &String) -> Vec<String> {
        vec![]
    }
}
