use crate::jav::sources::common::*;

const URL: &'static str = "https://indexav.com";

pub struct IndexAV;

lazy_static! {
    static ref CLIENT: reqwest::Client = reqwest::Client::new();
}

#[async_trait]
impl SearchByActress for IndexAV {
    async fn search_by_actress(actress: &String) -> Vec<AV> {
        let url = format!("{}/actor/{}", URL, actress);
        let body = CLIENT.get(&url).rsp_text().await;
        match body {
            Some(html) => {
                let soup = make_soup(html);

                soup.find(Class("video_column"))
                    .map(get_brief_from_card)
                    .collect()
            }
            None => vec![],
        }
    }
}

#[async_trait]
impl GetBrief for IndexAV {
    async fn get_brief(code: &String) -> Option<AV> {
        let url = format!("{}/search?keyword={}", URL, code);
        let body = CLIENT.get(&url).rsp_text().await?;
        let soup = make_soup(body);

        let cards: Vec<Node> = soup.find(Class("card")).collect();

        if cards.len() == 0 {
            return None;
        }

        if cards[0]
            .text()
            .contains("Sad, cannot find any video in database")
        {
            return None;
        }

        let result = get_brief_from_card(cards[0]);

        Some(result)
    }
}

fn get_brief_from_card(card: Node) -> AV {
    let mut res = AV::new();

    res.code = card
        .find(Class("tag").and(Class("is-link")).and(Class("is-light")))
        .next()
        .get_text();

    res.actress = card
        .find(Class("tag").and(Class("is-primary")).and(Class("is-light")))
        .map(|n| n.get_text().unwrap())
        .collect();

    let h5 = card.find(Class("title")).next();
    res.title = h5.get_text();
    if let Some(preview_img_url) = noexcept!(h5?.find(Name("a")).next()?.attr("rel")) {
        res.preview_img_url = Some(preview_img_url.to_string());
    }

    res.release_date = card
        .find(Class("footer"))
        .next()
        .and_then(|n| AV::strptime(n.as_text(), "%Y-%m-%d"));

    res
}
