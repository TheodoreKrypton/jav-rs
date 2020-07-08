use async_trait::async_trait;

use crate::jav::ds::AV;
use crate::jav::sources::traits::{GetBrief, SearchByActress};
use crate::{jav::sources::common::*, noexcept};

const URL: &'static str = "https://indexav.com";

pub struct IndexAV;

lazy_static! {
    static ref CLIENT: reqwest::Client = reqwest::Client::new();
}

#[async_trait]
impl SearchByActress for IndexAV {
    async fn search_by_actress(actress: String) -> Result<Vec<AV>, reqwest::Error> {
        let url = format!("{}/actor/{}", URL, actress);
        let body = CLIENT.get(&url).send().await?.text().await?;
        let soup = make_soup(body);

        Ok(soup
            .find(Class("video_column"))
            .map(get_brief_from_card)
            .collect())
    }
}

#[async_trait]
impl GetBrief for IndexAV {
    async fn get_brief(code: String) -> Result<Option<AV>, reqwest::Error> {
        let url = format!("{}/search?keyword={}", URL, code);
        let body = reqwest::get(&url).await?.text().await?;
        let soup = make_soup(body);

        let cards: Vec<Node> = soup.find(Class("card")).collect();

        if cards.len() == 0 {
            return Ok(None);
        }

        if cards[0]
            .text()
            .contains("Sad, cannot find any video in database")
        {
            return Ok(None);
        }

        let result = get_brief_from_card(cards[0]);

        Ok(Some(result))
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
    res.preview_img_url = noexcept!(Some(h5?.find(Name("a")).next()?.attr("rel")?.to_string()));

    res.release_date = AV::date_from_string(card.find(Class("footer")).next().get_text());

    res
}
