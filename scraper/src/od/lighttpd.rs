use super::all;
use super::olaindex::{OlaindexExtras, OLAINDEX};
use crate::parser;
use select::document::Document;
use select::predicate::{Class, Name, Predicate};

const IDENTIFIER: &str = "lighttpd";
pub struct LightTPD;
impl LightTPD {
    pub fn is_od(res: &str, server_name: &str) -> bool {
        LightTPD::footer_id(res)
            || LightTPD::script_title_id(res)
            || LightTPD::server_name_check(server_name)
    }
    /// Look at the `div.foot` for the id
    fn footer_id(res: &str) -> bool {
        Document::from(res)
            .find(Class("foot"))
            .any(|node| node.text().to_lowercase().starts_with(IDENTIFIER))
    }
    /// Look at the `div.script_title` for the id
    fn script_title_id(res: &str) -> bool {
        Document::from(res)
            .find(Class("script_title"))
            .any(|node| node.text().to_lowercase().starts_with(IDENTIFIER))
    }
    /// find the OD id through the `Server` Response header
    fn server_name_check(server_name: &str) -> bool {
        server_name.contains(IDENTIFIER)
    }
    /// Gets the names of the file and directory with the `/` included
    pub fn full_file_name(res: &str) -> Vec<String> {
        Document::from(res)
            .find(Name("tr").descendant(Name("td")))
            .filter_map(|node| Some(node.text()))
            .collect()
    }
    /// Parses lighttpd HTML Documents
    pub fn search(res: &str, url: &str) -> Vec<String> {
        let full_names = LightTPD::full_file_name(res);
        Document::from(res)
            .find(Name("tr").descendant(Name("td").descendant(Name("a"))))
            .filter(|node| all::no_parent_dir(url, &node.text(), node.attr("href")))
            .filter_map(|node| {
                let href = node.attr("href").unwrap();
                let new_href = format!("{}/", href);
                if full_names.contains(&new_href) {
                    Some(new_href)
                } else {
                    Some(href.to_string())
                }
            })
            .filter(|link| {
                let mut paths: Vec<&str> = link.split("/").collect();
                !OLAINDEX::has_extra_paths(&mut paths, OlaindexExtras::ExcludeHomeAndDownload)
            })
            .filter(|link| !link.contains("javascript:"))
            .map(|link| parser::sanitize_url(&link))
            .collect()
    }
}
