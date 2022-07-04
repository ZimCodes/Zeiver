use select::document::Document;
use select::predicate::Name;
const IDENTIFIER: &str = "LiteSpeed";

pub struct LiteSpeed;
impl LiteSpeed {
    pub fn is_od(res: &str, server: &str) -> bool {
        LiteSpeed::has_address(res) || server.contains(IDENTIFIER)
    }
    /// Check if ID is in address
    fn has_address(res: &str) -> bool {
        Document::from(res)
            .find(Name("address"))
            .any(|node| node.text().contains(IDENTIFIER))
    }
}
