use super::all;
use crate::parser;
use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};

const IDENTIFIER: &str = "PanIndex";
pub struct PanIndex;
impl PanIndex {
    pub fn is_od(res: &str) -> bool {
        PanIndex::copy_id(res) || PanIndex::toolbar_button_id(res)
    }
    /// Copyright Check
    fn copy_id(res: &str) -> bool {
        Document::from(res)
            .find(Name("div").and(Class("mdui-typo")))
            .any(|node| node.text().contains(IDENTIFIER))
    }
    /// Most common toolbar buttons
    fn toolbar_button_id(res: &str) -> bool {
        let mut buttons = vec![
            "brightness_5",
            "brightness_4",
            "panorama_wide_angle",
            "sort",
        ];
        Document::from(res)
            .find(
                Name("button")
                    .and(Attr("mdui-tooltip", ()))
                    .descendant(Name("i")),
            )
            .any(|node| {
                if buttons.len() == 1 {
                    true
                } else if buttons.contains(&&*node.text()) {
                    buttons.pop();
                    false
                } else {
                    false
                }
            })
    }
    /// Parses the PanIndex type ods
    pub fn search(res: &str, url: &str) -> Vec<String> {
        Document::from(res)
            .find(Name("div").and(Attr("data-url", ())))
            .filter(|node| all::no_parent_dir(url, &node.text(), node.attr("data-url")))
            .filter_map(|node| node.attr("data-url"))
            .filter(|link| !link.contains("javascript:"))
            .map(|link| parser::sanitize_url(link))
            .collect()
    }
}
