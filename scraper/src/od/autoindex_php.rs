use select::document::Document;
use select::predicate::{Name, Class, Not,Attr, Predicate};
use lazy_static::lazy_static;
use regex::Regex;
use url::Url;

const IDENTIFIER: &str = "AutoIndex PHP Script";
lazy_static! {
    static ref NAVIGATOR_REGEX:Regex = Regex::new(r"/[a-zA-Z]+\.php/?\?dir=").unwrap();
    static ref EXTENDED_NAV_REGEX:Regex = Regex::new(r"/?[a-zA-Z]+/[a-zA-Z]+\.php/?\?dir=").unwrap();
    static ref FILE_REGEX:Regex = Regex::new(r"&file=").unwrap();
    static ref BREADCRUMB:Regex = Regex::new(r"([a-zA-Z0-9 ]+$)").unwrap();
}
pub struct AutoIndexPHP;

impl AutoIndexPHP {
    /// [Identity] Identify whether this is an AutoIndex PHP OD
    pub fn is_od(res: &str) -> bool {
        let is_current_od = Document::from(res).find(Name("div").descendant(Name("a")
            .and(Attr("href","http://autoindex.sourceforge.net/")))
        ).any(|node| node.text().contains(IDENTIFIER));
        let breadcrumbs_exist = AutoIndexPHP::check_for_breadcrumbs(res);
        breadcrumbs_exist && is_current_od
    }
    /// Check if breadcrumbs exist
    fn check_for_breadcrumbs(res:&str)->bool{
        Document::from(res).find(Name("div").and(Not(Class("autoindex_small")))
            .descendant(Class("autoindex_a"))).any(|node| node.eq(&node))
    }
    /// Traverse document using standard `div:not(.autoindex_small) a.autoindex_a`
    fn standard_traversal(res: &str) -> Vec<String> {
        Document::from(res).find(Name("div").and(Not(Class("autoindex_small")))
            .descendant(Class("autoindex_a"))).map(|node| node.text()).collect()
    }
    /// Traverse document using special `div h2 a.default_a`
    fn special_default_traversal(res: &str) -> Vec<String> {
        Document::from(res).find(Name("div").descendant(Name("h2"))
            .descendant(Class("default_a"))).map(|node| node.text()).collect()
    }
    /// The starting path for downloading file
    fn start_path(res: &str) -> (String, bool) {
        let mut collection = AutoIndexPHP::standard_traversal(res);
        if collection.is_empty() {
            collection = AutoIndexPHP::special_default_traversal(res);
            AutoIndexPHP::retrieve_start_path(collection)
        } else {
            AutoIndexPHP::retrieve_start_path(collection)
        }
    }
    /// Retrieve the starting path
    fn retrieve_start_path(collection: Vec<String>) -> (String, bool) {
        let first_crumb = &collection[0];
        if first_crumb.starts_with("..") {
            let mut split_path: Vec<&str> = first_crumb.split("/").collect();
            if split_path.len() > 1 {
                split_path.remove(0);
                let trimmed_paths: Vec<&str> = split_path.iter().map(|path| path.trim()).collect();
                let path = format!("{}/",trimmed_paths.join("/"));
                if path.starts_with("/"){
                    (path, true)
                }else{
                    (format!("/{}",path), true)
                }
            } else {
                (String::new(), true)
            }
        } else if !first_crumb.ends_with(".") {
            let mut dir_split: Vec<&str> = first_crumb.split('/').collect();
            dir_split.remove(0);
            let trimmed_split: Vec<&str> = dir_split.iter().map(|path| path.trim()).collect();
            let path = format!("{}/",trimmed_split.join("/"));

            if path.starts_with("/"){
                (path, false)
            }else{
                (format!("/{}",path), false)
            }
        } else {
            (String::new(), false)
        }
    }
    /// [Transform] Transforms the file link into a downloadable one
    pub fn transform_dl_link(url: &str, rel: &str, res: &str) -> String {
        let (start_path, use_extend) = AutoIndexPHP::start_path(res);
        let no_nav_rel = match use_extend {
            true => EXTENDED_NAV_REGEX.replace(rel, start_path),
            false => NAVIGATOR_REGEX.replace(rel, start_path)
        };
        let filtered_rel = FILE_REGEX.replace(no_nav_rel.as_ref(), "");
        let url = Url::parse(url).expect(format!("Cannot parse url for transformation into download link: {}", url).as_str());
        let host = url.host_str().unwrap();
        let scheme = url.scheme();
        if filtered_rel.starts_with("//"){
            format!("{}://{}{}", scheme, host, &filtered_rel[1..])
        } else if  filtered_rel.starts_with("/"){
            format!("{}://{}{}", scheme, host, filtered_rel)
        } else {
            format!("{}://{}/{}", scheme, host, filtered_rel)
        }
    }
}