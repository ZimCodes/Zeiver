use super::all;
use crate::parser;
use select::document::Document;
use select::predicate::{Name, Predicate};

const IDENTIFIER: &str = "Caddy";

pub struct Caddy;

impl Caddy {
    pub fn is_od(res: &str, server: &str) -> bool {
        Caddy::footer_id(res) || server.contains(IDENTIFIER)
    }
    /// ID found in footer
    fn footer_id(res: &str) -> bool {
        Document::from(res)
            .find(Name("footer").descendant(Name("a")))
            .any(|node| node.text().to_lowercase() == IDENTIFIER.to_lowercase())
    }
    /// Parses Caddy ODs
    pub fn search(res: &str, url: &str) -> Vec<String> {
        Document::from(res)
            .find(Name("tr").descendant(Name("td").descendant(Name("a"))))
            .filter(|node| all::no_parent_dir(url, &node.text(), node.attr("href")))
            .filter_map(|node| node.attr("href"))
            .filter(|link| !link.contains("javascript:"))
            .map(|link| parser::sanitize_url(&link))
            .collect()
    }
}
