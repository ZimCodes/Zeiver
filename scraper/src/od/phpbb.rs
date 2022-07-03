use super::all;
use crate::parser;
use select::document::Document;
use select::predicate::{Attr, Name, Predicate};

const IDENTIFIER: &str = "phpBB";

pub struct PHPBB;
impl PHPBB {
    /// Check if appropriate od type
    pub fn is_od(res: &str) -> bool {
        PHPBB::selector_present(res) || PHPBB::description_id(res) || PHPBB::header_title(res)
    }
    /// Check if `body#phpbb` is present
    fn selector_present(res: &str) -> bool {
        Document::from(res)
            .find(Name("body").and(Attr("id", "phpbb")))
            .any(|node| node.eq(&node))
    }
    /// Check meta description tag for id
    fn description_id(res: &str) -> bool {
        Document::from(res)
            .find(Name("meta").and(Attr("name", "description")))
            .any(|node| match node.attr("content") {
                Some(content) => content.contains(IDENTIFIER),
                None => false,
            })
    }
    /// Find ID by looking at the Forum header
    fn header_title(res: &str) -> bool {
        Document::from(res)
            .find(Name("div").descendant(Name("h1")))
            .any(|node| node.text().contains(IDENTIFIER))
    }
    ///Filter out `AUTHORS` & `COPYING` files
    pub fn is_copy_file(x: &str) -> bool {
        x == "COPYING" || x == "AUTHORS"
    }
    /// Parses phpBB HTML Documents
    pub fn search(res: &str, url: &str) -> Vec<String> {
        let elements: Vec<String> = Document::from(res)
            .find(Name("tr").descendant(Name("td").descendant(Name("a"))))
            .filter(|node| all::no_parent_dir(url, &node.text(), node.attr("href")))
            .filter(|node| !PHPBB::is_copy_file(&node.text()))
            .filter_map(|node| node.attr("href"))
            .filter(|link| !parser::ends_with_any_query(link))
            .filter(|link| !link.contains("javascript:"))
            .map(|link| parser::sanitize_url(link))
            .collect();
        if !elements.is_empty() {
            elements
        } else {
            PHPBB::pre_search(res, url)
        }
    }
    fn pre_search(res: &str, url: &str) -> Vec<String> {
        let elements: Vec<String> = Document::from(res)
            .find(Name("pre").descendant(Name("a")))
            .filter(|node| all::no_parent_dir(url, &node.text(), node.attr("href")))
            .filter(|node| !PHPBB::is_copy_file(&node.text()))
            .filter_map(|node| node.attr("href"))
            .filter(|link| !parser::ends_with_any_query(link))
            .filter(|link| !link.contains("javascript:"))
            .map(|link| parser::sanitize_url(link))
            .collect();
        if !elements.is_empty() {
            elements
        } else {
            PHPBB::list_search(res, url)
        }
    }

    fn list_search(res: &str, url: &str) -> Vec<String> {
        Document::from(res)
            .find(Name("ul").descendant(Name("li").descendant(Name("a"))))
            .filter(|node| all::no_parent_dir(url, &node.text(), node.attr("href")))
            .filter(|node| !PHPBB::is_copy_file(&node.text()))
            .filter_map(|node| node.attr("href"))
            .filter(|link| !parser::ends_with_any_query(link))
            .filter(|link| !link.contains("javascript:"))
            .map(|link| parser::sanitize_url(link))
            .collect()
    }
}
