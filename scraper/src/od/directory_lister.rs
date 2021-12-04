use super::all;
use crate::parser;
use select::document::Document;
use select::predicate::{Class, Name, Predicate};

const IDENTIFIER: &str = "Directory Lister";

pub struct DirectoryLister;
impl DirectoryLister {
    pub fn is_od(res: &str) -> bool {
        DirectoryLister::footer_id(res) || DirectoryLister::icon_id(res)
    }
    fn footer_id(res: &str) -> bool {
        Document::from(res)
            .find(Name("footer").descendant(Name("p").descendant(Name("a"))))
            .any(|node| node.text() == IDENTIFIER)
    }
    fn icon_id(res: &str) -> bool {
        Document::from(res)
            .find(Class("fa-download"))
            .any(|node| node.eq(&node))
    }
    /// Parses the Directory Lister HTML Document type ods
    pub fn search(res: &str, url: &str) -> Vec<String> {
        Document::from(res)
            //Find all <a> tags
            .find(Name("ul").descendant(Name("a")))
            .filter(|node| {
                let link = node.attr("href").unwrap();
                !url.contains(link) && all::no_parent_dir(url, &node.text(), node.attr("href"))
            })
            .filter_map(|node| node.attr("href"))
            .filter(|link| !link.contains("javascript:"))
            .map(|link| parser::sanitize_url(link))
            .collect()
    }
}
