pub use async_trait::async_trait;
use html5ever;
use html5ever::tendril::StrTendril;
pub use lazy_static::lazy_static;
pub use regex::{Regex, RegexSet};
pub use reqwest;
use select;
use select::document::Document;
pub use select::predicate::{Attr, Class, Name, Predicate, Text};

pub fn make_soup(html: String) -> Document {
    Document::from(StrTendril::from(html))
}

pub trait GetText {
    fn get_text(&self) -> Option<String>;
}

pub type Node<'a> = select::node::Node<'a>;

impl<'a> GetText for Node<'a> {
    fn get_text(&self) -> Option<String> {
        Some(self.text().trim().to_string())
    }
}

impl<'a> GetText for Option<Node<'a>> {
    fn get_text(&self) -> Option<String> {
        match self {
            Some(n) => Some(n.text().trim().to_string()),
            None => None,
        }
    }
}

pub fn wrap_string(s: &String) -> Option<String> {
    match s.len() > 0 {
        true => Some(s.to_string()),
        false => None,
    }
}

#[async_trait]
pub trait ResponseText {
    async fn rsp_text(self) -> Option<String>;
}

#[async_trait]
impl ResponseText for reqwest::RequestBuilder {
    async fn rsp_text(self) -> Option<String> {
        Some(self.send().await.ok()?.text().await.ok()?)
    }
}

#[macro_export]
macro_rules! noexcept {
    ($x:expr) => {
        (|| -> Option<&str> { $x })()
    };
}
