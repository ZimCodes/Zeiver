use select::document::Document;
use select::predicate::{Predicate, Attr, Name, Class};

pub struct OdIndex;

impl OdIndex {
    pub fn is_od(res: &str) -> bool {
        OdIndex::has_potato_icon(res)
            || OdIndex::has_file_icon(res)
            || OdIndex::is_in_link_tag(res)
    }
    fn has_potato_icon(res: &str) -> bool {
        Document::from(res).find(Name("div")
            .and(Class("pathshow"))
            .descendant(Name("a"))).any(|element| {
            element.text().contains('🥔')
        })
    }
    fn has_file_icon(res:&str) -> bool{
        Document::from(res).find(Name("div").and(Class("headt"))).any(|element|{
            element.text().starts_with('📁')
        })
    }
    fn is_in_link_tag(res:&str) -> bool{
        Document::from(res).find(Name("link")
            .and(Attr("href",()))).any(|element| {
            match element.attr("href"){
                Some(href) => href.contains("OdIndex"),
                None => false
            }
        })
    }
    pub fn sanitize_url(x: &str) -> &str {
        if x.ends_with("?p=t") {
            &x[..x.len() - 4]
        }else {
             x
        }
    }
}