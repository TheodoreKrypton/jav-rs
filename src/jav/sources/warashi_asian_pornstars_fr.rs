use crate::jav::sources::traits::Translate2JP;
use crate::{jav::sources::common::*, noexcept};

pub struct WarashiAsianPornstarsFr;

const URL: &'static str = "http://warashi-asian-pornstars.fr";

lazy_static! {
    static ref CLIENT: reqwest::Client = reqwest::Client::new();
    static ref ACTRESS_DETAIL_URL: std::collections::HashMap<String, String> =
        std::collections::HashMap::new();
}

#[async_trait]
impl Translate2JP for WarashiAsianPornstarsFr {
    async fn translate2jp(name: String) -> Result<Option<String>, reqwest::Error> {
        let url = format!("{}/en/s-12/search", URL);
        let body = CLIENT
            .post(&url)
            .form(&[("recherche_critere", "f"), ("recherche_valeur", &name)])
            .send()
            .await?
            .text()
            .await?;
        let soup = make_soup(body);
        let name_lower = name.to_lowercase();
        let card = soup.find(Class("resultat-pornostar")).next();
        match card {
            Some(n) => Ok(get_name_in_card(&name_lower, &n)),
            None => Ok(None),
        }
    }
}

fn get_name_in_card(name: &str, card: &Node) -> Option<String> {
    let title = card.find(Name("p")).next()?.text().to_lowercase();
    if !title.contains(name) {
        return None;
    }
    let jp_name = title.split("-").nth(1)?.trim();
    if jp_name.len() == 0 {
        return None;
    }

    // cache for parsing actress info later, None for no url
    let url = noexcept!(Some(card.find(Name("a")).next()?.attr("href")?.to_string()));
    let detailed_url = format!("{}{}", URL, url.unwrap());

    Some(jp_name.to_string())
}
