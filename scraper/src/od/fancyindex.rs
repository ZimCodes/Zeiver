use lazy_static::lazy_static;
use regex::Regex;
use select::document::Document;
use select::predicate::{Attr, Comment, Name, Predicate};

lazy_static! {
    static ref ID_REGEX: Regex = Regex::new(r"[fF]ancy[iI]ndex").unwrap();
}
pub struct FancyIndex;
impl FancyIndex {
    pub fn is_od(res: &str) -> bool {
        FancyIndex::footer_id(res) || FancyIndex::comments(res) || FancyIndex::script_tags(res)
    }
    /// Check script tags for id
    fn script_tags(res: &str) -> bool {
        Document::from(res)
            .find(Name("script").and(Attr("src", ())))
            .any(|node| {
                let src = node.attr("src").unwrap();
                ID_REGEX.is_match(src)
            })
    }
    /// Check footer for id
    fn footer_id(res: &str) -> bool {
        Document::from(res)
            .find(Name("div").descendant(Name("small")))
            .any(|node| ID_REGEX.is_match(&node.text()))
    }
    ///Check Comments for id
    fn comments(res: &str) -> bool {
        Document::from(res)
            .find(Name("body").descendant(Comment))
            .any(|node| {
                if node.as_comment().is_none() {
                    false
                } else {
                    let comment = node.as_comment().unwrap();
                    ID_REGEX.is_match(comment)
                }
            })
    }
    //
}
