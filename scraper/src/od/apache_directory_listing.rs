use select::document::Document;
use select::predicate::{Attr, Name, Predicate};

const IDENTIFIER: &str = "Apache Directory Listing";
pub struct ApacheDirectoryListing;
impl ApacheDirectoryListing {
    pub fn is_od(res: &str) -> bool {
        ApacheDirectoryListing::id_tag(res) || ApacheDirectoryListing::footer(res)
    }
    /// Check for common table ID tag
    fn id_tag(res: &str) -> bool {
        Document::from(res)
            .find(Name("table").and(Attr("id", "indexlist")))
            .any(|node| node.eq(&node))
    }
    /// Check footer for id
    fn footer(res: &str) -> bool {
        Document::from(res)
            .find(Name("footer").descendant(Name("a").descendant(Name("em"))))
            .any(|node| node.text() == IDENTIFIER)
    }
}
