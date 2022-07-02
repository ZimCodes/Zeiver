use super::all;
use crate::parser;
use lazy_static::lazy_static;
use regex::Regex;
use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};
use url::Url;

const IDENTIFIER_CRUMB: &str = "Directory Listing of";
const IDENTIFIER_FOOTER: &str = "Web Design Sheffield";
const IDENTIFIER_TITLE: &str = "Evoluted Directory Listing Script";
lazy_static! {
    static ref NAVIGATOR_END_REGEX: Regex = Regex::new(r"/[a-zA-Z]+\.php/?\?dir=$").unwrap();
    static ref PHP_REGEX: Regex = Regex::new(r"/[a-zA-Z]+\.php/?$").unwrap();
}
pub struct DirectoryListingScript;
impl DirectoryListingScript {
    /// Check if ID of this OD can be found on this page
    pub fn is_od(res: &str) -> bool {
        let breadcrumb_id = Document::from(res)
            .find(Name("div").descendant(Name("h1")))
            .any(|node| node.text().contains(IDENTIFIER_CRUMB));
        breadcrumb_id
            || DirectoryListingScript::footer_id(res)
            || DirectoryListingScript::title_id(res)
    }
    /// Check if Id can be found in the bottom of the page
    fn footer_id(res: &str) -> bool {
        Document::from(res)
            .find(Name("div").descendant(Name("a")))
            .any(|node| node.text().contains(IDENTIFIER_FOOTER))
    }
    /// Check if ID can be found in the title of the page
    fn title_id(res: &str) -> bool {
        Document::from(res)
            .find(Name("div").descendant(Name("div").descendant(Name("h1"))))
            .any(|node| node.text().contains(IDENTIFIER_TITLE))
    }
    /// Check if url has `index.php?dir=` component at the end
    pub fn is_home_navigator(url: &str) -> bool {
        NAVIGATOR_END_REGEX.is_match(url)
    }
    /// [Transform] Transforms file links into a valid one
    pub fn transform_dir_link(rel: &str) -> String {
        rel.replace("%2F%2F", "%2F")
    }
    /// [Transform] Transforms file links into a valid one
    pub fn transform_link(url: &str, rel: &str) -> String {
        let url = Url::parse(url).expect(
            format!(
                "Cannot parse url for transformation into download link: {}",
                url
            )
            .as_str(),
        );
        let host = url.host_str().unwrap();
        if rel.contains(host) {
            return rel.to_string();
        }
        let scheme = url.scheme();
        let path = PHP_REGEX.replace(url.path(), "");
        if path.ends_with("/") && rel.starts_with("/") {
            format!("{}://{}{}{}", scheme, host, path, &rel[1..])
        } else if !path.ends_with("/") && !rel.starts_with("/") {
            format!("{}://{}{}/{}", scheme, host, path, rel)
        } else {
            format!("{}://{}{}{}", scheme, host, path, rel)
        }
    }
    /// Parses the Evoluted Directory Listing Script HTML Document type ods
    pub fn search(res: &str, url: &str) -> Vec<String> {
        Document::from(res)
            .find(
                Attr("id", "listingcontainer")
                    .descendant(Name("a"))
                    .or(Class("table-container").descendant(Name("a"))),
            )
            .filter(|node| all::no_parent_dir(url, &node.text(), node.attr("href")))
            .filter_map(|node| node.attr("href"))
            .filter(|link| !link.contains("javascript:"))
            .map(|link| link.replacen("?dir=.%2F", "?dir=./", 1))
            .map(|link| parser::sanitize_url(&link))
            .collect()
    }
}
