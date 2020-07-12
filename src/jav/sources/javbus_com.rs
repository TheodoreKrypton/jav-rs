use crate::jav::sources::common::*;

pub struct JavBusCom;

const URL: &'static str = "https://www.javbus.com";
const URL2: &'static str = "https://www.javbus.com/ajax/uncledatoolsbyajax.php";

lazy_static! {
    static ref CLIENT: reqwest::Client = reqwest::Client::new();
    static ref RE_GID: Regex = Regex::new(r"var gid = (\d+?);").unwrap();
    static ref RE_UC: Regex = Regex::new(r"var uc = (\d+?);").unwrap();
    static ref RE_IMG: Regex = Regex::new(r"var img = '(.+?)';").unwrap();
}

#[async_trait]
impl GetMagnets for JavBusCom {
    async fn get_magnets(code: &String) -> Vec<Magnet> {
        let url = format!("{}/{}", URL, code);
        if let Some(html) = CLIENT.get(&url).rsp_text().await {
            let query = {
                if let (Some(gid), Some(uc), Some(img)) = (
                    noexcept!(Some(RE_GID.captures(&html)?.get(1)?.as_str())),
                    noexcept!(Some(RE_UC.captures(&html)?.get(1)?.as_str())),
                    noexcept!(Some(RE_IMG.captures(&html)?.get(1)?.as_str())),
                ) {
                    [("gid", gid), ("lang", "zh"), ("uc", uc), ("img", img)]
                } else {
                    return vec![];
                }
            };

            if let Some(html) = CLIENT
                .get(URL2)
                .query(&query)
                .header("Referer", url)
                .rsp_text()
                .await
            {
                let soup = make_soup(format!("<table>{}</table>", html));
                return soup
                    .find(Name("tr"))
                    .map(|tr| -> Option<Magnet> {
                        if let Some(td) = tr.find(Name("td")).nth(1) {
                            let mut magnet = Magnet::new();
                            magnet.magnet = noexcept!(td.find(Name("a")).next()?.attr("href"))
                                .and_then(|s| Some(s.to_string()));
                            magnet.description = td.get_text();
                            return Some(magnet);
                        } else {
                            return None;
                        }
                    })
                    .filter(|m| m.is_some())
                    .map(|m| m.unwrap())
                    .collect();
            }
        }
        vec![]
    }
}
