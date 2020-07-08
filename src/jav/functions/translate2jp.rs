use crate::jav::ds::AV;
use crate::jav::sources::traits::Translate2JP;
use crate::jav::sources::WarashiAsianPornstarsFr;

pub async fn translate2jp(name: String) -> Option<String> {
    match WarashiAsianPornstarsFr::translate2jp(name).await {
        Ok(n) => n,
        Err(_) => None,
    }
}
