use select::document::Document;
use select::predicate::{Class, Name, Predicate};
pub struct OneIndex;

impl OneIndex {
    pub fn is_od(res: &str) -> bool {
        OneIndex::breadcrumb_id(res)
    }
    // Checks the second position of the breadcrumb for a `/`
    fn breadcrumb_id(res: &str) -> bool {
        let mut i = 0;
        Document::from(res)
            .find(Name("div").and(Class("mdui-toolbar")).descendant(Name("a")))
            .any(|node| {
                if i == 1 {
                    node.text().trim() == "/"
                } else {
                    i += 1;
                    false
                }
            })
    }
}
