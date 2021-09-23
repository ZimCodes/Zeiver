use regex::Regex;
use lazy_static::lazy_static;
use select::document::Document;
use select::predicate::{Name};
const IDENTIFIER:&str = "nginx";
lazy_static!{
    static ref SORT_QUERIES:Regex = Regex::new(r"\?[A-Z]=[A-Z]((;|&)[A-Z]=[A-Z])?$").unwrap();
}
pub struct NGINX;
impl NGINX{
    pub fn is_od(res:&str,server:&str) -> bool{
        let has_sort_queries = Document::from(res).find(Name("a")).any(|node| match node.attr("href"){
            Some(link)=> SORT_QUERIES.is_match(link),
            None => false
        });
         has_sort_queries || server.contains(IDENTIFIER)
    }
    pub fn has_extra_query(x:&str)->bool{
        SORT_QUERIES.is_match(x)
    }
}
#[cfg(test)]
mod tests{
    use super::SORT_QUERIES;
    #[test]
    fn sort_queries_regex(){
        assert_eq!(SORT_QUERIES.is_match("?C=N&O=A"),true);
        assert_eq!(SORT_QUERIES.is_match("?C=A&O=D"),true);
        assert_eq!(SORT_QUERIES.is_match("Text?C=A&O=D"),true);
        assert_eq!(SORT_QUERIES.is_match("Text/?C=A&O=D"),true);

        assert_eq!(SORT_QUERIES.is_match("Text/?C=A&O=DFinish"),false);
    }
}