extern crate chrono;
extern crate serde;

use chrono::NaiveDate;
use serde::Serialize;

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
    pub fn new() -> Self {
        AV {
            code: None,
            video_url: None,
            preview_img_url: None,
            actress: vec![],
            release_date: None,
            title: None,
        }
    }

    pub fn strptime(release_date: Option<&str>, format: &str) -> Option<NaiveDate> {
        NaiveDate::parse_from_str(&release_date?, format).ok()
    }
}
