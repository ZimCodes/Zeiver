use select::document::Document;
use select::predicate::Name;

/// Checks if HTML Document is an OD
pub fn is_invalid_od(res:&str)->bool{
    has_no_script(res) && !has_anchors(res)
}
/// Check Document for an `<noscript>`
fn has_no_script(res:&str) ->bool{
    Document::from(res).find(Name("noscript"))
        .any(|node| node.eq(&node))
}
/// Check Document for `<a>`
fn has_anchors(res:&str)->bool{
    Document::from(res).find(Name("a"))
        .any(|node| node.eq(&node))
}