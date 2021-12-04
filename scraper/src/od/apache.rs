use super::all;
use super::nginx::NGINX;
use crate::parser;
use select::document::Document;
use select::predicate::{Name, Predicate};

const IDENTIFIER: &str = "Apache";

pub struct Apache;
impl Apache {
    pub fn is_od(res: &str, server: &str) -> bool {
        let is_od = server.contains(IDENTIFIER);
        if !is_od {
            Apache::address_check(res)
        } else {
            true
        }
    }
    /// Check for id in the address tag
    fn address_check(res: &str) -> bool {
        Document::from(res)
            .find(Name("address"))
            .any(|node| node.text().contains(IDENTIFIER))
    }

    /// Parses the Apache & NGINX HTML Document type ods
    pub fn search(res: &str, url: &str) -> Vec<String> {
        Document::from(res)
            .find(
                Name("tr")
                    .descendant(Name("td").descendant(Name("a")))
                    .or(Name("pre").descendant(Name("a")))
                    .or(Name("li").descendant(Name("a"))),
            )
            .filter(|node| all::no_parent_dir(url, &node.text(), node.attr("href")))
            .filter(|node| !NGINX::has_extra_query(node.attr("href").unwrap()))
            .filter_map(|node| node.attr("href"))
            .filter(|link| !link.contains("javascript:"))
            .map(|link| parser::sanitize_url(link))
            .collect()
    }
}
