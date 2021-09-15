pub mod olaindex;
pub mod autoindex_php;

#[derive(PartialEq, Debug)]
pub enum ODMethod {
    OLAINDEX,
    AutoIndexPHP,
    Generic,
}
/*Determine the od type from URL*/
pub fn od_type_from_url(url: &str) -> ODMethod {
    if olaindex::OLAINDEX::hash_query(url) {
        ODMethod::OLAINDEX
    } else {
        ODMethod::Generic
    }
}
/*Determine od type from HTML Document */
pub fn od_type_from_document(res: &str) -> ODMethod {
    if olaindex::OLAINDEX::has_data_route(res) {
        ODMethod::OLAINDEX
    } else if autoindex_php::AutoIndexPHP::is_od(res) {
        ODMethod::AutoIndexPHP
    } else {
        ODMethod::Generic
    }
}
