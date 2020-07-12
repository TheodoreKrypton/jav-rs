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
struct ApiResponse {
    response: _Response,
}

pub struct AvgleCom;

lazy_static! {
    static ref CLIENT: reqwest::Client = reqwest::Client::new();
}

#[async_trait]
impl SearchByCode for AvgleCom {
    async fn search_by_code(code: &String) -> Option<AV> {
        let url = format!("https://api.avgle.com/v1/search/{}/0?limit=1", code);
        let body = CLIENT.get(&url).rsp_text().await?;
        if let Ok(rsp) = serde_json::from_str::<ApiResponse>(&body) {
            if rsp.response.videos.len() != 0 {
                let video = &(rsp.response.videos[0]);
                let mut res = AV::new();
                res.code = Some(code.to_string());
                res.title = wrap_string(&video.title);
                res.preview_img_url = wrap_string(&video.preview_url);
                res.video_url = wrap_string(&video.video_url);
                return Some(res);
            }
        }
        return None;
    }
}
