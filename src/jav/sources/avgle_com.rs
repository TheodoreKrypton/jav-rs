use serde::Deserialize;

use crate::jav::ds::AV;
use crate::jav::sources::common::*;

#[derive(Deserialize)]
struct _Video {
    title: String,
    video_url: String,
    preview_url: String,
}

#[derive(Deserialize)]
struct _Response {
    videos: Vec<_Video>,
}

#[derive(Deserialize)]
struct AvgleApiResponse {
    response: _Response,
}

pub async fn search_by_code(code: &str) -> Result<Option<AV>, reqwest::Error> {
    let url = format!("https://api.avgle.com/v1/search/{}/0?limit=1", code);
    let body = reqwest::get(&url).await?.text().await?;

    println!("{}", url);

    let rsp: Option<AvgleApiResponse> = serde_json::from_str(&body).ok();
    match rsp {
        Some(r) => {
            if r.response.videos.len() == 0 {
                return Ok(None);
            } else {
                let video = &(r.response.videos[0]);
                let mut res = AV::new();
                res.code = Some(code.to_string());

                res.preview_img_url = wrap_string(&video.preview_url);
                res.video_url = wrap_string(&video.video_url);
                return Ok(Some(res));
            }
        }

        None => {
            return Ok(None);
        }
    }
}
