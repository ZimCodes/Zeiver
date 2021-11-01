use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};

const IDENTIFIER_ONE: &str = "h5ai";
const IDENTIFIER_TWO: &str = "_h5ai";
pub struct H5AI;
impl H5AI {
    pub fn is_od(res: &str) -> bool {
        H5AI::topbar_title_id(res)
            || H5AI::bottom_title_id(res)
            || H5AI::tree_id(res)
            || H5AI::meta_id(res)
            || H5AI::css_id(res)
            || H5AI::js_id(res)
    }
    /// Check the topbar for the ID
    fn topbar_title_id(res: &str) -> bool {
        Document::from(res)
            .find(
                Name("div")
                    .and(Attr("id", "topbar"))
                    .descendant(Name("a").and(Attr("id", "backlink"))),
            )
            .any(|node| match node.attr("title") {
                Some(title) => title.contains(IDENTIFIER_ONE),
                None => false,
            })
    }
    /// Check the bottom bar for the ID
    fn bottom_title_id(res: &str) -> bool {
        Document::from(res)
            .find(
                Name("div")
                    .and(Attr("id", "bottombar"))
                    .descendant(Name("span").and(Class("right")))
                    .descendant(Name("a")),
            )
            .any(|node| node.text().contains(IDENTIFIER_ONE))
    }
    /// Locate the h5ai sidebar tree
    fn tree_id(res: &str) -> bool {
        Document::from(res)
            .find(Name("div").and(Attr("id", "tree")))
            .any(|node| node.eq(&node))
    }
    /// Search the meta tag for an ID
    fn meta_id(res: &str) -> bool {
        Document::from(res)
            .find(Name("meta").and(Attr("name", "description")))
            .any(|node| match node.attr("content") {
                Some(content) => content.contains(IDENTIFIER_ONE),
                None => false,
            })
    }
    /// Find ID through CSS file name
    fn css_id(res: &str) -> bool {
        Document::from(res)
            .find(Name("link").and(Attr("rel", "stylesheet")))
            .any(|node| match node.attr("href") {
                Some(link) => link.contains(IDENTIFIER_TWO),
                None => false,
            })
    }
    /// Find the ID through the JS file name
    fn js_id(res: &str) -> bool {
        Document::from(res)
            .find(Name("script"))
            .any(|node| match node.attr("src") {
                Some(src) => src.contains(IDENTIFIER_TWO),
                None => false,
            })
    }
}
