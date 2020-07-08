use crate::jav::ds::AV;
use crate::jav::sources::traits::SearchByActress;
use crate::jav::sources::IndexAV;

pub async fn search_by_actress(code: String) -> Vec<AV> {
    match IndexAV::search_by_actress(code).await {
        Ok(avs) => avs,
        Err(_) => vec![],
    }
}
