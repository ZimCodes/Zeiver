use lazy_static::lazy_static;
use regex::Regex;
use select::document::Document;
use select::predicate::Name;
const IDENTIFIER: &str = "nginx";
lazy_static! {
    static ref SORT_QUERIES: Regex =
        Regex::new(r"\?[a-zA-Z]=[a-zA-Z]((;|&)[a-zA-Z]=[a-zA-Z])?/?$").unwrap();
}
pub struct NGINX;
impl NGINX {
    pub fn is_od(res: &str, server: &str) -> bool {
        let has_sort_queries =
            Document::from(res)
                .find(Name("a"))
                .any(|node| match node.attr("href") {
                    Some(link) => SORT_QUERIES.is_match(link),
                    None => false,
                });
        has_sort_queries || server.contains(IDENTIFIER)
    }
    pub fn has_extra_query(x: &str) -> bool {
        SORT_QUERIES.is_match(x)
    }
    /// Remove sort queries
    pub fn sanitize_url(x: &str) -> String {
        String::from(SORT_QUERIES.replace(x, ""))
    }
}
#[cfg(test)]
mod tests {
    use super::SORT_QUERIES;
    #[test]
    fn sort_queries_regex() {
        assert_eq!(SORT_QUERIES.is_match("?C=N&O=A"), true);
        assert_eq!(SORT_QUERIES.is_match("?c=n&o=a"), true);
        assert_eq!(SORT_QUERIES.is_match("?C=n&o=a"), true);
        assert_eq!(SORT_QUERIES.is_match("?c=N&o=a"), true);
        assert_eq!(SORT_QUERIES.is_match("?c=n&O=a"), true);
        assert_eq!(SORT_QUERIES.is_match("?c=n&o=A"), true);
        assert_eq!(SORT_QUERIES.is_match("?C=A&O=D"), true);
        assert_eq!(SORT_QUERIES.is_match("?C=A"), true);
        assert_eq!(SORT_QUERIES.is_match("?C=a"), true);
        assert_eq!(SORT_QUERIES.is_match("?c=a"), true);
        assert_eq!(SORT_QUERIES.is_match("Text?C=A&O=D"), true);
        assert_eq!(SORT_QUERIES.is_match("Text/?C=A&O=D"), true);

        assert_eq!(SORT_QUERIES.is_match("Text/?C=AD"), false);
        assert_eq!(SORT_QUERIES.is_match("Text/?CA"), false);
        assert_eq!(SORT_QUERIES.is_match("Text/?CD=A"), false);
        assert_eq!(SORT_QUERIES.is_match("Text/?C=A&O=DFinish"), false);
        assert_eq!(SORT_QUERIES.is_match("Text/?CL=A&O=DFinish"), false);
        assert_eq!(SORT_QUERIES.is_match("Text/?CL=AD&O=DFinish"), false);
        assert_eq!(SORT_QUERIES.is_match("Text/?CA&O=DFinish"), false);
        assert_eq!(SORT_QUERIES.is_match("Text/?CA=A&OD=DFinish"), false);
        assert_eq!(SORT_QUERIES.is_match("?C=A&&O=D"), false);
    }
}
