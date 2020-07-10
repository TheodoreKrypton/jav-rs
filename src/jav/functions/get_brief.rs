use crate::jav::{
    ds::AV,
    sources::{
        traits::{ropt, GetBrief},
        IndexAV, WarashiAsianPornstarsFr,
    },
};

use futures::try_join;

pub async fn get_brief(code: &String) -> Option<AV> {
    let result = try_join!(
        // ropt(IndexAV::get_brief(code)),
        ropt(WarashiAsianPornstarsFr::get_brief(code))
    );
    match result {
        Ok(_) => None,
        Err(r) => Some(r),
    }
}
