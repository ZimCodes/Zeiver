use select::document::Document;
use select::predicate::Name;

const IDENTIFIER: &str = "Apache";

pub struct Apache;
impl Apache {
    pub fn is_od(res: &str, server: &str) -> bool {
        let is_od = server.contains(IDENTIFIER);
        if !is_od {
            Apache::address_check(res)
        } else {
            true
        }
    }
    /// Check for id in the address tag
    fn address_check(res: &str) -> bool {
        Document::from(res)
            .find(Name("address"))
            .any(|node| node.text().contains(IDENTIFIER))
    }
}
