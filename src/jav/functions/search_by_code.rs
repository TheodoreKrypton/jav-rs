use crate::jav::ds::AV;
use crate::jav::sources::traits::SearchByCode;
use crate::jav::sources::Avgle;

pub async fn search_by_code(code: &String) -> Option<AV> {
    return Avgle::search_by_code(code).await;
}
