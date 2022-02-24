use super::{all, nginx::NGINX};
use crate::parser;
use select::document::Document;
use select::predicate::{Attr, Comment, Name, Not, Predicate};
const REPO_URL: &str = "https://github.com/desandro/windex";
const ID_PATH: &str = "/windex";
pub struct Windex;
impl Windex {
    pub fn is_od(res: &str) -> bool {
        Windex::comments(res) || Windex::style_tags(res) || Windex::script_tags(res)
    }
    /// Check Comments for repo source
    pub fn comments(res: &str) -> bool {
        Document::from(res)
            .find(Name("head").descendant(Comment))
            .any(|node| {
                if node.as_comment().is_none() {
                    false
                } else {
                    let comment = node.as_comment().unwrap();
                    comment.contains(REPO_URL)
                }
            })
    }
    /// Check stylesheets for id
    pub fn style_tags(res: &str) -> bool {
        Document::from(res)
            .find(Name("link").and(Attr("href", ())))
            .any(|node| {
                if node.attr("href").is_none() {
                    false
                } else {
                    let href = node.attr("href").unwrap();
                    href.contains(ID_PATH)
                }
            })
    }
    /// Check script tags for id
    pub fn script_tags(res: &str) -> bool {
        Document::from(res)
            .find(Name("script").and(Attr("src", ())))
            .any(|node| {
                if node.attr("src").is_none() {
                    false
                } else {
                    let src = node.attr("src").unwrap();
                    src.contains(ID_PATH)
                }
            })
    }
    /// Parses the usual HTML Document type ods
    pub fn search(res: &str, url: &str) -> Vec<String> {
        Document::from(res)
            //Find all <a> tags
            .find(
                Name("tr").descendant(
                    Name("td")
                        .and(Not(Attr("valign", "top")))
                        .descendant(Name("a")),
                ),
            )
            .filter(|node| all::no_parent_dir(url, &node.text(), node.attr("href")))
            .filter(|node| !NGINX::has_extra_query(node.attr("href").unwrap()))
            .filter_map(|node| node.attr("href"))
            .filter(|link| !link.contains("javascript:"))
            .map(|link| parser::sanitize_url(link))
            .collect()
    }
}
