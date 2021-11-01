use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};

const IDENTIFIER: &str = "OneManager";
const IDENTIFIER_BUTTON: &str = "CopyAllDownloadUrl";
const IDENTIFIER_EXPAND_ICON: &str = "folder_open";
const IDENTIFIER_FOLDER_ICON: &str = "expand_more";

pub struct OneManager;

impl OneManager {
    pub fn is_od(res: &str) -> bool {
        OneManager::html_class_id(res)
            || OneManager::meta_id(res)
            || OneManager::title_id(res)
            || OneManager::download_button_id(res)
            || OneManager::icons_id(res)
    }
    /// Check if `html.hydrated` selector is present
    fn html_class_id(res: &str) -> bool {
        Document::from(res)
            .find(Name("html").and(Class("hydrated")))
            .any(|node| node.eq(&node))
    }
    /// Check the keywords meta tag for id
    fn meta_id(res: &str) -> bool {
        Document::from(res)
            .find(Name("meta").and(Attr("name", "keywords")))
            .any(|node| node.text().contains(IDENTIFIER))
    }
    /// Check the title for id
    fn title_id(res: &str) -> bool {
        Document::from(res)
            .find(Name("h1").descendant(Name("a")))
            .any(|node| node.text().contains(IDENTIFIER))
    }
    /// Determine if download button exists
    fn download_button_id(res: &str) -> bool {
        Document::from(res)
            .find(Name("tr").and(Attr("id", "tr0").descendant(Name("button"))))
            .any(|node| node.text().contains(IDENTIFIER_BUTTON))
    }
    /// Determine if expand_more and folder_open icons exist
    fn icons_id(res: &str) -> bool {
        Document::from(res)
            .find(Name("i"))
            .any(|node| node.text().contains(IDENTIFIER_EXPAND_ICON))
            || Document::from(res)
                .find(Name("i"))
                .any(|node| node.text().contains(IDENTIFIER_FOLDER_ICON))
    }
}
