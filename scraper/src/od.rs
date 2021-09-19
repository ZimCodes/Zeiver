pub mod olaindex;
pub mod autoindex_php;
pub mod directory_lister;

#[derive(PartialEq, Debug)]
pub enum ODMethod {
    OLAINDEX,
    AutoIndexPHP,
    AutoIndexPHPNoCrumb,
    DirectoryLister,
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
    if olaindex::OLAINDEX::is_od(res) {
        ODMethod::OLAINDEX
    } else if directory_lister::DirectoryLister::is_od(res){
        ODMethod::DirectoryLister
    }else {
        let (breadcrumb_exist, is_autoindex) = autoindex_php::AutoIndexPHP::is_od(res);
        if breadcrumb_exist && is_autoindex {
            ODMethod::AutoIndexPHP
        }else if is_autoindex {
            ODMethod::AutoIndexPHPNoCrumb
        }else {
            ODMethod::Generic
        }
    }
}
