use select::document::Document;
use select::predicate::{Class, Name, Predicate};
const IDENTIFIER: &str = "snif";
pub struct Snif;
impl Snif {
    pub fn is_od(res: &str) -> bool {
        Snif::copyright_id(res) || Snif::selector_id(res)
    }
    /// Unique copyright ID
    fn copyright_id(res: &str) -> bool {
        Document::from(res)
            .find(Name("div").and(Class("snCopyright")).descendant(Name("a")))
            .any(|node| node.text().starts_with(IDENTIFIER))
    }
    /// Presence of the snif class selector
    fn selector_id(res: &str) -> bool {
        Document::from(res)
            .find(Name("body").and(Class("snif")))
            .any(|node| node.eq(&node))
    }
    /// Filter out the parent directory
    pub fn is_parent(x: Option<&str>) -> bool {
        match x {
            Some(title) => title == "..",
            None => false,
        }
    }
    /// Filter out extra download links
    pub fn is_download(x: Option<&str>) -> bool {
        match x {
            Some(link) => link.contains("&download="),
            None => false,
        }
    }
}
