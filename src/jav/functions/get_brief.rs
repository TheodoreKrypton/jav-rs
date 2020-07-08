use crate::jav::ds::AV;
use crate::jav::sources::traits::GetBrief;
use crate::jav::sources::IndexAV;

pub async fn get_brief(code: String) -> Option<AV> {
    match IndexAV::get_brief(code).await {
        Ok(av) => av,
        Err(_) => None,
    }
}
