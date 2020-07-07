extern crate html5ever;
extern crate select;

use select::document::Document;

use html5ever::tendril::StrTendril;

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

#[macro_export]
macro_rules! noexcept {
    ($x:expr) => {
        (|| -> Option<String> { $x })()
    };
}
