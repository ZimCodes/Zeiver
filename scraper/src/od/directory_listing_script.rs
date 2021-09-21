use select::document::Document;
use select::predicate::{Predicate,Name};
use regex::Regex;
use lazy_static::lazy_static;

const IDENTIFIER_CRUMB:&str = "Directory Listing of";
const IDENTIFIER_FOOTER:&str = "Web Design Sheffield";
const IDENTIFIER_TITLE:&str = "Evoluted Directory Listing Script";
lazy_static!{
    static ref NAVIGATOR_END_REGEX:Regex =  Regex::new(r"/[a-zA-Z]+\.php/?\?dir=$").unwrap();
}
pub struct DirectoryListingScript;
impl DirectoryListingScript{
    /// Check if ID of this OD can be found on this page
    pub fn is_od(res:&str)-> bool{
        let breadcrumb_id = Document::from(res).find(Name("div").descendant(Name("h1")))
            .any(|node| node.text().contains(IDENTIFIER_CRUMB));
        breadcrumb_id || DirectoryListingScript::footer_id(res) || DirectoryListingScript::title_id(res)
    }
    /// Check if Id can be found in the bottom of the page
    fn footer_id(res:&str)->bool{
        Document::from(res).find(Name("div").descendant(Name("a")))
            .any(|node| node.text().contains(IDENTIFIER_FOOTER))
    }
    /// Check if ID can be found in the title of the page
    fn title_id(res:&str)->bool{
        Document::from(res).find(Name("div").descendant(Name("div").descendant(Name("h1"))))
            .any(|node| node.text().contains(IDENTIFIER_TITLE))
    }
    /// Check if url has `index.php?dir=` component at the end
    pub fn is_home_navigator(url:&str) -> bool{
        NAVIGATOR_END_REGEX.is_match(url)
    }
}