pub mod olaindex;
pub mod autoindex_php;
pub mod directory_lister;
pub mod apache;
pub mod nginx;

#[derive(PartialEq, Debug)]
pub enum ODMethod {
    OLAINDEX,
    AutoIndexPHP,
    AutoIndexPHPNoCrumb,
    DirectoryLister,
    Apache,
    NGINX,
    Generic,
    None,
}

///Determine the od type from URL
pub fn od_type_from_url(url: &str) -> ODMethod {
    if olaindex::OLAINDEX::hash_query(url) {
        ODMethod::OLAINDEX
    } else {
        ODMethod::None
    }
}

/// Determine od type from HTML Document
pub fn od_type_from_document(res: &str, server_name: &str) -> ODMethod {
    if olaindex::OLAINDEX::is_od(res) {
        ODMethod::OLAINDEX
    } else if directory_lister::DirectoryLister::is_od(res) {
        ODMethod::DirectoryLister
    } else {
        od_type_from_header(res, server_name)
    }
}

/// Determine OD Type from `Server` header
fn od_type_from_header(res: &str, server_name: &str) -> ODMethod{
    if apache::Apache::is_od(res, server_name) {
        ODMethod::Apache
    } else if nginx::NGINX::is_od(server_name) {
        ODMethod::NGINX
    } else {
        final_type_check(res)
    }
}

/// The final resort to checking od type
fn final_type_check(res: &str) -> ODMethod {
    let (breadcrumb_exist, is_autoindex) = autoindex_php::AutoIndexPHP::is_od(res);
    if breadcrumb_exist && is_autoindex {
        ODMethod::AutoIndexPHP
    } else if is_autoindex {
        ODMethod::AutoIndexPHPNoCrumb
    } else {
        ODMethod::Generic
    }
}