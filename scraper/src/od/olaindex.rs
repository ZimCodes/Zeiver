use super::all;
use crate::parser;
use lazy_static::lazy_static;
use regex::Regex;
use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};
use std::borrow::Cow;

const IDENTIFIER_DESCRIPTION: &str = "OLAINDEX,Another OneDrive Directory Index";
const IDENTIFIER_KEYWORDS: &str = "OLAINDEX,OneDrive,Index,Microsoft OneDrive,Directory Index";

lazy_static! {
    static ref OLAINDEX_HASH_QUERY: Regex =
        Regex::new(r"\?hash=[0-9a-zA-Z]{8}(/?|&download=1)$").unwrap();
    static ref OLAINDEX_QUERIES: Regex = Regex::new(r"\?hash=[0-9a-zA-Z]{8}&download=1$").unwrap();
    static ref OLAINDEX_EXTRA_PATHS: Regex =
        Regex::new(r"(/|%2F)(v|view|d|down|home|s|show)(/|%2F)").unwrap();
}
pub enum OlaindexExtras {
    All,
    ExcludeHomeAndDownload,
}
pub struct OLAINDEX;

impl OLAINDEX {
    /// Adds a download query to end of file url
    /// [Transform]
    pub fn transform_link(x: &str) -> String {
        if !OLAINDEX::has_dl_query(x) {
            format!("{}?download=1", x)
        } else {
            x.to_string()
        }
    }
    /// Check to see if url has download query to the OLAINDEX ODs
    /// Ex: https://example.com/coolthing.mp4?download=1
    pub fn has_dl_query(x: &str) -> bool {
        x.ends_with("?download=1")
    }
    ///Custom version for files to Accept/Reject (OLAINDEX)
    pub fn acc_rej_filters(
        accept: &Option<String>,
        reject: &Option<String>,
    ) -> (Option<String>, Option<String>) {
        let new_accept = if accept.is_some() {
            Some(format!(r"{}(\?download=1$)", accept.as_ref().unwrap()))
        } else {
            None
        };
        let new_reject = if reject.is_some() {
            Some(format!(r"{}(\?download=1$)", reject.as_ref().unwrap()))
        } else {
            None
        };
        (new_accept, new_reject)
    }
    /// Checks for hash query in URL
    /// [Identity]
    pub fn hash_query(x: &str) -> bool {
        OLAINDEX_HASH_QUERY.is_match(x)
    }
    /// Removes hash query
    pub fn sanitize_url(url: &str) -> Cow<str> {
        OLAINDEX_HASH_QUERY.replace(url, "")
    }
    /// Check if path contains any extra paths such as '/d/,/down/,/v/,/view/,/home/,etc.'
    pub fn has_extra_paths(paths: &mut Vec<&str>, include: OlaindexExtras) -> bool {
        let extra_path = paths.get(3);

        match extra_path {
            Some(path) => {
                let is_common_search = path == &"v" || path == &"view";
                let is_down_search = path == &"d" || path == &"down";
                let is_show_search = path == &"s" || path == &"show";
                let is_home = path == &"home";
                match include {
                    OlaindexExtras::ExcludeHomeAndDownload => is_common_search || is_show_search,
                    _ => is_common_search || is_down_search || is_show_search || is_home,
                }
            }
            None => false,
        }
    }
    /// Removes extra paths from broken down url
    pub fn remove_extra_paths(paths: &mut Vec<&str>, include: OlaindexExtras) {
        if OLAINDEX::has_extra_paths(paths, include) {
            paths.remove(3);
        }
    }
    /// Removes an extra path from an path
    pub fn remove_path(path: &str) -> String {
        OLAINDEX_EXTRA_PATHS.replace(path, "/").to_string()
    }
    /// [Identity] Used to determine od type
    pub fn is_od(res: &str) -> bool {
        let has_description_id = Document::from(res)
            .find(
                Name("meta")
                    .and(Attr("name", "description").and(Attr("content", IDENTIFIER_DESCRIPTION))),
            )
            .any(|node| node.eq(&node));
        if !has_description_id {
            Document::from(res)
                .find(
                    Name("meta")
                        .and(Attr("name", "keywords").and(Attr("content", IDENTIFIER_KEYWORDS))),
                )
                .any(|node| node.eq(&node))
        } else {
            has_description_id
        }
    }
    /// Parses the OLAINDEX HTML Document type ods
    pub fn search(res: &str, url: &str) -> Vec<String> {
        Document::from(res)
            .find(
                Name("div")
                    .and(Class("mdui-container").or(Class("container")))
                    .descendant(Name("a").or(Name("li"))),
            )
            .filter(|node| all::no_parent_dir(url, &node.text(), node.attr("href")))
            .filter_map(|node| {
                if node.attr("data-route").is_some() {
                    node.attr("data-route")
                } else {
                    node.attr("href")
                }
            })
            .filter(|link| {
                let mut paths: Vec<&str> = link.split("/").collect();
                !OLAINDEX::has_extra_paths(&mut paths, OlaindexExtras::ExcludeHomeAndDownload)
            })
            .filter(|link| !link.contains("javascript:"))
            .map(|link| parser::sanitize_url(link))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::OLAINDEX_EXTRA_PATHS;
    #[test]
    fn extra_paths_regex() {
        const HOME: &str = "https://www.example.org";
        assert_eq!(OLAINDEX_EXTRA_PATHS.is_match(&format!("{}/v/", HOME)), true);
        assert_eq!(
            OLAINDEX_EXTRA_PATHS.is_match(&format!("{}/v%2F", HOME)),
            true
        );
        assert_eq!(
            OLAINDEX_EXTRA_PATHS.is_match(&format!("{}%2Fv/", HOME)),
            true
        );
        assert_eq!(
            OLAINDEX_EXTRA_PATHS.is_match(&format!("{}%2Fv%2F", HOME)),
            true
        );
        assert_eq!(OLAINDEX_EXTRA_PATHS.is_match(&format!("{}/s/", HOME)), true);
        assert_eq!(
            OLAINDEX_EXTRA_PATHS.is_match(&format!("{}/show/", HOME)),
            true
        );
        assert_eq!(
            OLAINDEX_EXTRA_PATHS.is_match(&format!("{}/view/", HOME)),
            true
        );
        assert_eq!(
            OLAINDEX_EXTRA_PATHS.is_match(&format!("{}/home/", HOME)),
            true
        );
        assert_eq!(OLAINDEX_EXTRA_PATHS.is_match(&format!("{}/d/", HOME)), true);
        assert_eq!(
            OLAINDEX_EXTRA_PATHS.is_match(&format!("{}/down/", HOME)),
            true
        );

        assert_eq!(
            OLAINDEX_EXTRA_PATHS.is_match(&format!("{}/down", HOME)),
            false
        );
        assert_eq!(
            OLAINDEX_EXTRA_PATHS.is_match(&format!("{}/t/", HOME)),
            false
        );
        assert_eq!(OLAINDEX_EXTRA_PATHS.is_match(&format!("{}t/", HOME)), false);
        assert_eq!(OLAINDEX_EXTRA_PATHS.is_match(&format!("{}t", HOME)), false);
        assert_eq!(
            OLAINDEX_EXTRA_PATHS.is_match(&format!("{}t%2F", HOME)),
            false
        );
        assert_eq!(
            OLAINDEX_EXTRA_PATHS.is_match(&format!("{}%2Ftime", HOME)),
            false
        );
    }
}
