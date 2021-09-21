use select::document::Document;
use select::predicate::{Predicate, Name, Class};

const IDENTIFIER:&str = "Directory Lister";

pub struct DirectoryLister;
impl DirectoryLister{
    pub fn is_od(res:&str)->bool{
        DirectoryLister::footer_id(res) || DirectoryLister::icon_id(res)
    }
    fn footer_id(res:&str) ->bool{
        Document::from(res).find(Name("footer").descendant(Name("p").descendant(Name("a"))))
            .any(|node| node.text() == IDENTIFIER)
    }
    fn icon_id(res:&str)->bool{
        Document::from(res).find(Class("fa-download"))
            .any(|node| node.eq(&node))
    }
}