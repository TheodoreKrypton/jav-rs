use crate::jav::ds::AV;
use crate::jav::sources::traits::SearchByActress;
use crate::jav::sources::IndexAV;

pub async fn search_by_actress(code: &String) -> Vec<AV> {
    return IndexAV::search_by_actress(code).await;
}
