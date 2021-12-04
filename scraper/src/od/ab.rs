use super::all;
use crate::parser;
use select::document::Document;
use select::predicate::{Class, Name, Predicate};

const IDENTIFIER: &str = "A.B.";
pub struct AB;
impl AB {
    pub fn is_od(res: &str) -> bool {
        AB::footer(res)
    }
    /// footer slogan id
    fn footer(res: &str) -> bool {
        Document::from(res)
            .find(Name("body").descendant(Name("p")))
            .any(|node| {
                node.text().contains(IDENTIFIER) && node.text().contains("Page generated in")
            })
    }

    /// Parses A.B HTML Document type ods
    pub fn search(res: &str, url: &str) -> Vec<String> {
        Document::from(res)
            .find(
                Name("tbody")
                    .descendant(Name("td").and(Class("item")))
                    .descendant(Name("a")),
            )
            .filter(|node| all::no_parent_dir(url, &node.text(), node.attr("href")))
            .filter_map(|node| node.attr("href"))
            .filter(|link| !link.contains("javascript:"))
            .map(|link| parser::sanitize_url(link))
            .collect()
    }
}
