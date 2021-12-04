use super::all;
use super::oneindex::OneIndex;
use crate::parser;
use select::document::Document;
use select::predicate::{Attr, Class, Comment, Name, Not, Predicate};

const IDENTIFIER: &str = "OneManager";
const IDENTIFIER_BUTTON: &str = "CopyAllDownloadUrl";

pub struct OneManager;

impl OneManager {
    pub fn is_od(res: &str) -> bool {
        OneManager::html_class_id(res)
            || OneManager::comment_id(res)
            || OneManager::meta_id(res)
            || OneManager::title_id(res)
            || OneManager::download_button_id(res)
    }
    /// Check if `html.hydrated` selector is present
    fn html_class_id(res: &str) -> bool {
        Document::from(res)
            .find(Name("html").and(Class("hydrated")))
            .any(|node| node.eq(&node))
    }
    // Check a comment referencing link to repo
    fn comment_id(res: &str) -> bool {
        Document::from(res)
            .find(Comment)
            .any(|node| match node.as_comment() {
                Some(comment) => comment.contains(IDENTIFIER),
                None => false,
            })
    }
    /// Check the keywords meta tag for id
    fn meta_id(res: &str) -> bool {
        Document::from(res)
            .find(Name("meta").and(Attr("name", "keywords")))
            .any(|node| match node.attr("content") {
                Some(content) => content.contains(IDENTIFIER),
                None => false,
            })
    }
    /// Check the title for id
    fn title_id(res: &str) -> bool {
        Document::from(res)
            .find(Name("h1").descendant(Name("a")))
            .any(|node| node.text().contains(IDENTIFIER))
    }
    /// Determine if download button exists
    fn download_button_id(res: &str) -> bool {
        Document::from(res)
            .find(Name("tr").and(Attr("id", "tr0").descendant(Name("button"))))
            .any(|node| node.text().contains(IDENTIFIER_BUTTON))
    }
    /// Parses Modern OneManager HTML Documents
    pub fn search(res: &str, url: &str) -> Vec<String> {
        let links: Vec<String> = Document::from(res)
            .find(
                Name("td")
                    .and(Class("file"))
                    .descendant(Name("a").and(Not(Class("download")))),
            )
            .filter(|node| all::no_parent_dir(url, &node.text(), node.attr("href")))
            .filter(|node| !node.text().contains("arrow_"))
            .filter_map(|node| node.attr("href"))
            .filter(|link| !link.ends_with("/?/"))
            .filter(|link| !link.contains("javascript:"))
            .map(|link| parser::sanitize_url(link))
            .collect();
        if links.is_empty() {
            OneIndex::search(res, url)
        } else {
            links
        }
    }
}
