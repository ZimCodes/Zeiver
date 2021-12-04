use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};

const WGET_ID: &str = "[Copy] WGET";
const IDENTIFIER: &str = "eyy-indexer";
pub struct EyyIndexer;
impl EyyIndexer {
    pub fn is_od(res: &str) -> bool {
        EyyIndexer::wget_copy(res) || EyyIndexer::footer_items(res) || EyyIndexer::od_id(res)
    }
    /// WGet copy command
    fn wget_copy(res: &str) -> bool {
        Document::from(res)
            .find(
                Name("div")
                    .and(Class("menu"))
                    .descendant(Name("div").and(Attr("id", "copy"))),
            )
            .any(|node| node.text() == WGET_ID)
    }
    ///OD id
    fn od_id(res: &str) -> bool {
        Document::from(res)
            .find(
                Name("div")
                    .and(Class("bottom"))
                    .descendant(Name("div").and(Class("git-reference")))
                    .descendant(Name("a")),
            )
            .any(|node| node.text() == IDENTIFIER)
    }
    /// Key footer items
    fn footer_items(res: &str) -> bool {
        Document::from(res)
            .find(Name("div").and(Class("bottom")))
            .any(|node| {
                node.text().contains("Page generated in") && node.text().contains("Browsing")
            })
    }
}
