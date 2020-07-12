use crate::jav::sources::common::*;

pub struct AVHelpMemoWiki;

#[async_trait]
impl GetAliases for AVHelpMemoWiki {
    async fn get_aliases(actress: &String) -> Vec<String> {
        vec![]
    }
}
