use std::{collections::HashMap, sync::Mutex};

use crate::{
    jav::{
        ds::AV,
        sources::{common::*, traits::*},
    },
    noexcept,
};

pub struct WarashiAsianPornstarsFr;

const URL: &'static str = "http://warashi-asian-pornstars.fr";

lazy_static! {
    static ref CLIENT: reqwest::Client = reqwest::Client::new();
    static ref ACTRESS_DETAIL_URL: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

#[async_trait]
impl Translate2JP for WarashiAsianPornstarsFr {
    async fn translate2jp(name: &String) -> Option<String> {
        let url = format!("{}/en/s-12/search", URL);
        let body = CLIENT
            .post(&url)
            .form(&[("recherche_critere", "f"), ("recherche_valeur", &name)])
            .rsp_text()
            .await?;
        let soup = make_soup(body);
        let name_lower = name.to_lowercase();
        if let Some(card) = soup.find(Class("resultat-pornostar")).next() {
            get_name_in_card(&name_lower, &card)
        } else {
            None
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
    let url = noexcept!(card.find(Name("a")).next()?.attr("href"));
    let detailed_url = format!("{}{}", URL, url.unwrap());

    ACTRESS_DETAIL_URL
        .lock()
        .unwrap()
        .insert(name.to_string(), detailed_url.clone());
    ACTRESS_DETAIL_URL
        .lock()
        .unwrap()
        .insert(jp_name.to_string(), detailed_url);

    Some(jp_name.to_string())
}

#[async_trait]
impl GetBrief for WarashiAsianPornstarsFr {
    async fn get_brief(code: &String) -> Option<AV> {
        let url = format!("{}/en/s-12/search", URL);

        let url = {
            let body = CLIENT
                .post(&url)
                .form(&[("recherche_critere", "v"), ("recherche_valeur", &code)])
                .rsp_text()
                .await?;
            let soup = make_soup(body);
            let div = soup.find(Class("resultat-film")).next()?;
            format!("{}{}", URL, div.find(Name("a")).next()?.attr("href")?)
        };

        let soup = {
            let body = CLIENT.get(&url).rsp_text().await?;
            make_soup(body)
        };

        let ps = {
            let div = soup.find(Attr("id", "fiche-film-infos")).next()?;
            div.find(Name("p"))
        };

        let mut result = AV::new();

        if let Some(poster) = noexcept!(soup.find(Name("video")).next()?.attr("poster")) {
            result.preview_img_url = Some(format!("{}{}", URL, poster));
        }

        result.code = Some(code.to_string());

        for p in ps {
            let text = p.text();
            if !text.contains(":") {
                continue;
            }

            let tokens: Vec<&str> = text.split(":").collect();
            if tokens.len() != 2 {
                continue;
            }

            if let [k, v] = tokens[..] {
                match k {
                    "original title" => {
                        result.title = Some(v.trim().to_string());
                    }
                    "release date" => {
                        result.release_date = AV::strptime(Some(v.trim()), "%B %d, %Y");
                    }
                    &_ => {}
                }
            }
        }

        result.actress = {
            match soup.find(Attr("id", "casting-f")).next() {
                Some(div) => div
                    .find(Class("ja"))
                    .map(|p| p.get_text())
                    .filter(|s| s.is_some())
                    .map(|s| s.unwrap())
                    .collect(),
                None => vec![],
            }
        };

        return Some(result);
    }
}
