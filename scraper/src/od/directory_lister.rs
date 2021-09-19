use select::document::Document;
use select::predicate::{Predicate,Name};


const IDENTIFIER:&str = "Directory Lister";

pub struct DirectoryLister;
impl DirectoryLister{
    pub fn is_od(res:&str)->bool{
        Document::from(res).find(Name("footer").descendant(Name("p").descendant(Name("a"))))
            .any(|node| node.text() == IDENTIFIER)
    }
}