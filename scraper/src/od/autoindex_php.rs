use super::all;
use crate::parser;
use lazy_static::lazy_static;
use regex::Regex;
use select::document::Document;
use select::predicate::{Class, Name, Not, Predicate};
use url::Url;

const IDENTIFIER: &str = "AutoIndex PHP Script";
lazy_static! {
    static ref NAVIGATOR_REGEX: Regex = Regex::new(r"/[a-zA-Z0-9]+\.php/?\?dir=").unwrap();
    static ref EXTENDED_NAV_REGEX: Regex =
        Regex::new(r"/?[a-zA-Z0-9]+/[a-zA-Z0-9]+\.php/?\?dir=").unwrap();
    static ref HTML_NAV_EXTENDED: Regex =
        Regex::new(r"/?[a-zA-Z0-9]+/[a-zA-Z0-9]+\.html/?$").unwrap();
    static ref HTML_NAV_REGEX: Regex = Regex::new(r"/[a-zA-Z0-9]+\.html/?$").unwrap();
    static ref FILE_REGEX: Regex = Regex::new(r"&file=").unwrap();
    static ref BREADCRUMB: Regex = Regex::new(r"([a-zA-Z0-9 ]+$)").unwrap();
    static ref HTML_REGEX: Regex = Regex::new(r"/[a-zA-Z0-9\-_]+\.html$").unwrap();
    static ref DIR_QUERY: Regex = Regex::new(r"\?dir=.*$").unwrap();
}
pub struct AutoIndexPHP;

impl AutoIndexPHP {
    /// [Identity] Identify whether this is an AutoIndex PHP OD
    pub fn is_od(res: &str) -> (bool, bool) {
        let is_current_od = Document::from(res)
            .find(Name("a"))
            .any(|node| node.text().contains(IDENTIFIER));
        let breadcrumbs_exist = AutoIndexPHP::check_for_breadcrumbs(res);
        (breadcrumbs_exist, is_current_od)
    }
    /// Check if breadcrumbs exist
    fn check_for_breadcrumbs(res: &str) -> bool {
        Document::from(res)
            .find(
                Name("div")
                    .and(Not(Class("autoindex_small")))
                    .descendant(Class("autoindex_a").or(Class("default_a"))),
            )
            .any(|node| node.eq(&node))
    }

    /// Traverse document using standard for breadcrumb `div:not(.autoindex_small) a.autoindex_a`
    fn traversal(res: &str) -> Vec<String> {
        let breadcrumbs: Vec<String> = Document::from(res)
            .find(
                Name("div")
                    .and(Not(Class("autoindex_small")))
                    .descendant(Class("autoindex_a")),
            )
            .map(|node| node.text())
            .collect();
        if breadcrumbs.is_empty() {
            AutoIndexPHP::table_traversal(res)
        } else {
            breadcrumbs
        }
    }
    /// Traverse document using special `div:not(.autoindex_small) .default_a` for breadcrumb
    fn table_traversal(res: &str) -> Vec<String> {
        let breadcrumbs: Vec<String> = Document::from(res)
            .find(
                Name("div")
                    .and(Not(Class("autoindex_small")))
                    .descendant(Class("default_a")),
            )
            .map(|node| node.text())
            .collect();
        if breadcrumbs.is_empty() {
            AutoIndexPHP::special_traversal(res)
        } else {
            breadcrumbs
        }
    }

    /// Traverse document using special `div h2 a.default_a` for breadcrumb
    fn special_traversal(res: &str) -> Vec<String> {
        Document::from(res)
            .find(
                Name("div")
                    .descendant(Name("h2"))
                    .descendant(Class("default_a")),
            )
            .map(|node| node.text())
            .collect()
    }
    /// The starting path for downloading file
    fn start_path(res: &str) -> (String, bool, bool) {
        let collection = AutoIndexPHP::traversal(res);
        AutoIndexPHP::retrieve_start_path(collection)
    }
    /// Retrieve the starting path of the breadcrumb
    fn retrieve_start_path(collection: Vec<String>) -> (String, bool, bool) {
        let first_crumb: &str = &collection[0].trim();
        // Used to cancel the retrieval of the breadcrumbs when the OD
        // doesn't have one.
        if first_crumb.to_lowercase() == IDENTIFIER.to_lowercase() {
            return (String::new(), true, true);
        }
        if first_crumb.starts_with("..") {
            let mut split_path: Vec<&str> = first_crumb.split('/').collect();
            if split_path.len() > 1 {
                split_path.remove(0);
                let trimmed_paths: Vec<&str> = split_path.iter().map(|path| path.trim()).collect();
                let path = format!("{}/", trimmed_paths.join("/"));
                if path.starts_with("/") {
                    (path, true, false)
                } else {
                    (format!("/{}", path), true, false)
                }
            } else {
                (String::new(), true, false)
            }
        } else if !first_crumb.ends_with(".") {
            let mut dir_split: Vec<&str> = first_crumb.split('/').collect();
            dir_split.remove(0);
            let trimmed_split: Vec<&str> = dir_split.iter().map(|path| path.trim()).collect();
            let path = format!("{}/", trimmed_split.join("/"));

            if path.starts_with("/") {
                (path, false, false)
            } else {
                (format!("/{}", path), false, false)
            }
        } else if first_crumb == "." {
            (String::from("/"), false, false)
        } else {
            (String::new(), false, false)
        }
    }
    /// [Transform] Transforms the file link into a downloadable one
    pub fn transform_dl_link(url: &str, rel: &str, res: &str) -> String {
        let url = Url::parse(url).expect(
            format!(
                "Cannot parse url for transformation into download link: {}",
                url
            )
            .as_str(),
        );
        if url.path().ends_with("php") {
            AutoIndexPHP::transform_php_dl_link(url, rel, res)
        } else {
            AutoIndexPHP::transform_html_dl_link(url, rel)
        }
    }
    /// [Transform] Transforms file link into a downloadable one (PHP)
    fn transform_php_dl_link(url: Url, rel: &str, res: &str) -> String {
        let (start_path, use_extend, is_cancelled) = AutoIndexPHP::start_path(res);
        if is_cancelled {
            return rel.to_string();
        }
        let no_nav_rel = match use_extend {
            true => EXTENDED_NAV_REGEX.replace(rel, start_path),
            false => NAVIGATOR_REGEX.replace(rel, start_path),
        };
        let filtered_rel = FILE_REGEX.replace(no_nav_rel.as_ref(), "");

        let host = url.host_str().unwrap();
        let scheme = url.scheme();
        if filtered_rel.starts_with("//") {
            format!("{}://{}{}", scheme, host, &filtered_rel[1..])
        } else if filtered_rel.starts_with("/") {
            format!("{}://{}{}", scheme, host, filtered_rel)
        } else {
            format!("{}://{}/{}", scheme, host, filtered_rel)
        }
    }
    /// [Transform] Transforms file link into a downloadable one (HTML)
    fn transform_html_dl_link(url: Url, rel: &str) -> String {
        let host = url.host_str().unwrap();
        let scheme = url.scheme();
        let path = HTML_NAV_EXTENDED.replace(url.path(), "");
        let filtered_rel = String::from(rel.trim_start_matches('.'));
        let rel_path = DIR_QUERY.replace(&filtered_rel, "");
        if rel_path.starts_with("/") {
            format!("{}://{}{}{}", scheme, host, path, rel_path)
        } else {
            format!("{}://{}{}/{}", scheme, host, path, rel_path)
        }
    }
    /// [Transform] Transforms the directory link into a valid one (HTML)
    pub fn transform_dir_link_html(url: &str, rel: &str) -> String {
        let url = Url::parse(url).expect(
            format!(
                "Cannot parse url for transformation into download link: {}",
                url
            )
            .as_str(),
        );

        if !url.path().ends_with("html") {
            return rel.to_string();
        }
        let host = url.host_str().unwrap();
        let scheme = url.scheme();
        let path = HTML_NAV_REGEX.replace(url.path(), "");
        if rel.starts_with("/") {
            format!("{}://{}{}{}", scheme, host, path, rel)
        } else {
            format!("{}://{}{}/{}", scheme, host, path, rel)
        }
    }
    /// Parses the AutoIndex PHP HTML Document type ods
    pub fn search(res: &str, url: &str) -> Vec<String> {
        Document::from(res)
            .find(Name("tbody").descendant(Class("autoindex_a").or(Class("default_a"))))
            .filter(|node| all::no_parent_dir(url, &node.text(), node.attr("href")))
            .filter_map(|node| node.attr("href"))
            .filter(|link| !link.contains("javascript:"))
            .map(|link| parser::sanitize_url(link))
            .collect()
    }
}
