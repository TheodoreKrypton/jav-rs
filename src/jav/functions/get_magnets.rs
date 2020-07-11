use crate::jav::{
    ds::Magnet,
    sources::{traits::GetMagnets, JavBusCom},
};

pub async fn get_magnets(code: &String) -> Vec<Magnet> {
    JavBusCom::get_magnets(code).await
}
