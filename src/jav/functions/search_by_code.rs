use crate::jav::ds::AV;
use crate::jav::sources::traits::SearchByCode;
use crate::jav::sources::Avgle;

pub async fn search_by_code(code: String) -> Option<AV> {
    match Avgle::search_by_code(code).await {
        Ok(av) => av,
        Err(_) => None,
    }
}
