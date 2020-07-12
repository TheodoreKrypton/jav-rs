use crate::jav::sources::common::*;

pub struct JavMostCom;

const URL: &'static str = "http://www5.javmost.com";

#[derive(Deserialize)]
struct NewlyReleasedResponse {
    data: String,
}

lazy_static! {
    static ref CLIENT: reqwest::Client = reqwest::Client::new();
    static ref RE_DATE: Regex = Regex::new(r"(\d\d\d\d-\d\d-\d\d)").unwrap();
}

#[async_trait]
impl GetNewlyReleased for JavMostCom {
    async fn get_newly_released(page: u32) -> Vec<AV> {
        let url = format!("{}/showlist/new/{}/release", URL, page);
        if let Some(json) = CLIENT.get(&url).rsp_text().await {
            if let Ok(rsp) = serde_json::from_str::<NewlyReleasedResponse>(&json) {
                let soup = make_soup(rsp.data);
                let cards = soup.find(Name("div").and(Class("card")));
                return cards.map(get_brief_from_card).collect::<Vec<AV>>();
            }
        }
        vec![]
    }
}

fn get_brief_from_card(card: Node) -> AV {
    let mut av = AV::new();

    if let Some(icon) = card.find(Class("fa-calendar")).next() {
        av.release_date = card
            .descendants()
            .nth(icon.index() - card.index())
            .and_then(|n| match n.as_text() {
                Some(s) => AV::strptime(
                    RE_DATE
                        .captures(s)
                        .and_then(|c| noexcept!(Some(c.get(1)?.as_str()))),
                    "%Y-%m-%d",
                ),
                None => None,
            });
    }

    av.actress = card
        .find(Name("a").and(Class("btn-danger")))
        .map(|a| a.get_text().unwrap())
        .collect::<Vec<String>>();

    av.preview_img_url = noexcept!(card.find(Name("img")).next()?.attr("data-src"))
        .and_then(|s| Some(s.to_string()));
    av.title = card.find(Name("h5")).next().get_text();
    av.code = card.find(Name("h4")).next().get_text();
    av
}
