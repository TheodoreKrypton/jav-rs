use crate::jav::{
    ds::AV,
    sources::{traits::GetNewlyReleased, JavMostCom},
};

pub async fn get_newly_released(page: u32) -> Vec<AV> {
    JavMostCom::get_newly_released(page).await
}
