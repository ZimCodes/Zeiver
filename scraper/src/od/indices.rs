use super::all;
use crate::parser;
use select::document::Document;
use select::predicate::{Attr, Class, Comment, Name, Predicate};

const SOURCE_URL: &str = "http://antisleep.com/software/indices";
pub struct Indices;
impl Indices {
    pub fn is_od(res: &str) -> bool {
        Indices::outside_comment(res)
            || Indices::style_comment(res)
            || Indices::source_type_a(res)
            || Indices::source_type_other(res)
    }
    /// Comment outside of html DOM
    pub fn outside_comment(res: &str) -> bool {
        Document::from(res).find(Comment).any(|node| {
            if node.as_comment().is_none() {
                false
            } else {
                let comment = node.as_comment().unwrap();
                comment.contains(SOURCE_URL)
            }
        })
    }
    /// Style comment
    pub fn style_comment(res: &str) -> bool {
        Document::from(res)
            .find(Name("head").descendant(Name("style").descendant(Comment)))
            .any(|node| {
                if node.as_comment().is_none() {
                    false
                } else {
                    let comment = node.as_comment().unwrap();
                    comment.contains("Indices styles:")
                }
            })
    }
    /// Source Website for type A's
    pub fn source_type_a(res: &str) -> bool {
        Document::from(res)
            .find(
                Name("div")
                    .and(Class("span10"))
                    .descendant(Name("p").descendant(Name("a").and(Attr("href", ())))),
            )
            .any(|node| {
                if node.attr("href").is_none() {
                    false
                } else {
                    let href = node.attr("href").unwrap();
                    href.contains(SOURCE_URL)
                }
            })
    }
    /// Source Website for regular types
    pub fn source_type_other(res: &str) -> bool {
        Document::from(res)
            .find(
                Name("div")
                    .and(Class("credits"))
                    .descendant(Name("a").and(Attr("href", ()))),
            )
            .any(|node| {
                if node.attr("href").is_none() {
                    false
                } else {
                    let href = node.attr("href").unwrap();
                    href.contains(SOURCE_URL)
                }
            })
    }

    /// Parses the Indices HTML Document type ods
    pub fn search(res: &str, url: &str) -> Vec<String> {
        Document::from(res)
            .find(Name("tr").descendant(Name("td").descendant(Name("a"))))
            .filter(|node| all::no_parent_dir(url, &node.text(), node.attr("href")))
            .filter_map(|node| node.attr("href"))
            .filter(|link| !link.contains("javascript:"))
            .map(|link| parser::sanitize_url(link))
            .collect()
    }
}
