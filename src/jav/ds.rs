extern crate chrono;
extern crate serde;

use chrono::NaiveDate;
use serde::Serialize;

const FORMAT: &'static str = "%Y-%m-%d";

#[derive(Serialize, Debug)]
pub struct AV {
    pub code: Option<String>,
    pub video_url: Option<String>,
    pub preview_img_url: Option<String>,
    pub actress: Vec<String>,
    pub release_date: Option<NaiveDate>,
    pub title: Option<String>,
}

impl AV {
    pub fn new() -> AV {
        AV {
            code: None,
            video_url: None,
            preview_img_url: None,
            actress: vec![],
            release_date: None,
            title: None,
        }
    }

    pub fn date_from_string(release_date: Option<String>) -> Option<NaiveDate> {
        NaiveDate::parse_from_str(&release_date?, FORMAT).ok()
    }
}
