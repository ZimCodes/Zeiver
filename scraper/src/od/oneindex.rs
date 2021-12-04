use super::all;
use crate::parser;
use select::document::Document;
use select::predicate::{Attr, Class, Name, Not, Predicate};

pub struct OneIndex;

impl OneIndex {
    pub fn is_od(res: &str) -> bool {
        OneIndex::breadcrumb_id(res)
    }
    // Checks the second position of the breadcrumb for a `/`
    fn breadcrumb_id(res: &str) -> bool {
        let mut i = 0;
        Document::from(res)
            .find(Name("div").and(Class("mdui-toolbar")).descendant(Name("a")))
            .any(|node| {
                if i == 1 {
                    node.text().trim() == "/"
                } else {
                    i += 1;
                    false
                }
            })
    }
    /// Parses OneIndex related HTML Documents
    pub fn search(res: &str, url: &str) -> Vec<String> {
        Document::from(res)
            .find(
                Name("div").and(Class("mdui-container")).descendant(
                    Name("li").descendant(
                        Name("a").and(
                            Not(Attr("title", "download")).and(Not(Attr("title", "直接下载"))),
                        ),
                    ),
                ),
            )
            .filter(|node| all::no_parent_dir(url, &node.text(), node.attr("href")))
            .filter(|node| !node.text().contains("arrow_"))
            .filter_map(|node| node.attr("href"))
            .filter(|link| !link.ends_with("/?/"))
            .filter(|link| !link.contains("javascript:"))
            .map(|link| parser::sanitize_url(link))
            .collect()
    }
}
