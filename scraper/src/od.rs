pub mod olaindex;
pub mod autoindex_php;
pub mod directory_lister;
pub mod apache;
pub mod nginx;
pub mod directory_listing_script;
pub mod lighttpd;
pub mod phpbb;
mod none;

#[derive(PartialEq, Debug)]
pub enum ODMethod {
    OLAINDEX,
    AutoIndexPHP,
    AutoIndexPHPNoCrumb,
    DirectoryLister,
    Apache,
    NGINX,
    DirectoryListingScript,
    LightTPD,
    PHPBB,
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
    if none::is_invalid_od(res) {
        ODMethod::None
    } else if olaindex::OLAINDEX::is_od(res) {
        ODMethod::OLAINDEX
    } else if directory_lister::DirectoryLister::is_od(res) {
        ODMethod::DirectoryLister
    } else if directory_listing_script::DirectoryListingScript::is_od(res) {
        ODMethod::DirectoryListingScript
    } else if phpbb::PHPBB::is_od(res){
        ODMethod::PHPBB
    }else {
        autoindex_type_check(res, server_name)
    }
}

/// AutoIndex od type check
fn autoindex_type_check(res: &str, server_name: &str) -> ODMethod {
    let (breadcrumb_exist, is_autoindex) = autoindex_php::AutoIndexPHP::is_od(res);
    if breadcrumb_exist && is_autoindex {
        ODMethod::AutoIndexPHP
    } else if is_autoindex {
        ODMethod::AutoIndexPHPNoCrumb
    } else {
        od_type_from_header(res, server_name)
    }
}

/// Determine OD Type from `Server` header
fn od_type_from_header(res: &str, server_name: &str) -> ODMethod {
    if lighttpd::LightTPD::is_od(res, server_name) {
        ODMethod::LightTPD
    } else if apache::Apache::is_od(res, server_name) {
        ODMethod::Apache
    } else if nginx::NGINX::is_od(server_name) {
        ODMethod::NGINX
    } else {
        ODMethod::Generic
    }
}