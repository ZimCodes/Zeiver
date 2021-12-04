use super::all;
use crate::parser;
use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};

pub struct OdIndex;

impl OdIndex {
    pub fn is_od(res: &str) -> bool {
        OdIndex::has_potato_icon(res) || OdIndex::has_file_icon(res) || OdIndex::is_in_link_tag(res)
    }
    fn has_potato_icon(res: &str) -> bool {
        Document::from(res)
            .find(Name("div").and(Class("pathshow")).descendant(Name("a")))
            .any(|element| element.text().contains('🥔'))
    }
    fn has_file_icon(res: &str) -> bool {
        Document::from(res)
            .find(Name("div").and(Class("headt")))
            .any(|element| element.text().starts_with('📁'))
    }
    fn is_in_link_tag(res: &str) -> bool {
        Document::from(res)
            .find(Name("link").and(Attr("href", ())))
            .any(|element| match element.attr("href") {
                Some(href) => href.contains("OdIndex"),
                None => false,
            })
    }
    pub fn sanitize_url(x: &str) -> &str {
        if x.ends_with("?p=t") {
            &x[..x.len() - 4]
        } else {
            x
        }
    }
    /// Parses OdIndex HTML Documents
    pub fn search(res: &str, url: &str) -> Vec<String> {
        Document::from(res)
            .find(Name("a").and(Class("item")))
            .filter(|node| all::no_parent_dir(url, &node.text(), node.attr("href")))
            .filter(|node| !node.text().ends_with(".."))
            .filter_map(|node| node.attr("href"))
            .filter(|link| !link.contains("javascript:"))
            .map(|link| parser::sanitize_url(link))
            .collect()
    }
}
