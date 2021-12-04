use super::all;
use crate::parser;
use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};

const IDENTIFIER: &str = "Apache Directory Listing";
pub struct ApacheDirectoryListing;
impl ApacheDirectoryListing {
    pub fn is_od(res: &str) -> bool {
        ApacheDirectoryListing::id_tag(res) || ApacheDirectoryListing::footer(res)
    }
    /// Check for common table ID tag
    fn id_tag(res: &str) -> bool {
        Document::from(res)
            .find(Name("table").and(Attr("id", "indexlist")))
            .any(|node| node.eq(&node))
    }
    /// Check footer for id
    fn footer(res: &str) -> bool {
        Document::from(res)
            .find(Name("footer").descendant(Name("a").descendant(Name("em"))))
            .any(|node| node.text() == IDENTIFIER)
    }
    /// Parses Apache Directory Listing HTML Document ods
    pub fn search(res: &str, url: &str) -> Vec<String> {
        Document::from(res)
            .find(
                Name("table")
                    .and(Attr("id", "indexlist"))
                    .descendant(Name("tr"))
                    .descendant(Name("td").and(Class("indexcolname")))
                    .descendant(Name("a")),
            )
            .filter(|node| all::no_parent_dir(url, &node.text(), node.attr("href")))
            .filter_map(|node| node.attr("href"))
            .filter(|link| !link.contains("javascript:"))
            .map(|link| parser::sanitize_url(link))
            .collect()
    }
}
