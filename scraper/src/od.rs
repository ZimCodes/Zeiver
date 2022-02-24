pub mod ab;
pub mod all;
pub mod apache;
pub mod apache_directory_listing;
pub mod autoindex_php;
pub mod directory_lister;
pub mod directory_listing_script;
pub mod eyy_indexer;
pub mod fancyindex;
pub mod h5ai;
pub mod indices;
pub mod lighttpd;
pub mod microsoftiis;
pub mod nginx;
mod none;
pub mod odindex;
pub mod olaindex;
pub mod oneindex;
pub mod onemanager;
pub mod panindex;
pub mod phpbb;
pub mod snif;
pub mod windex;

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
    OneManager,
    H5AI,
    MicrosoftIIS,
    Snif,
    OdIndex,
    OneIndex,
    PanIndex,
    ApacheDirectoryListing,
    EyyIndexer,
    FancyIndex,
    AB,
    Generic,
    None,
    Windex,
    Indices,
}

/// Determine the od type from URL
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
    } else if phpbb::PHPBB::is_od(res) {
        ODMethod::PHPBB
    } else if oneindex::OneIndex::is_od(res) {
        ODMethod::OneIndex
    } else if onemanager::OneManager::is_od(res) {
        ODMethod::OneManager
    } else if h5ai::H5AI::is_od(res) {
        ODMethod::H5AI
    } else if snif::Snif::is_od(res) {
        ODMethod::Snif
    } else if odindex::OdIndex::is_od(res) {
        ODMethod::OdIndex
    } else if panindex::PanIndex::is_od(res) {
        ODMethod::PanIndex
    } else if indices::Indices::is_od(res) {
        ODMethod::Indices
    } else if windex::Windex::is_od(res) {
        ODMethod::Windex
    } else if apache_directory_listing::ApacheDirectoryListing::is_od(res) {
        ODMethod::ApacheDirectoryListing
    } else if eyy_indexer::EyyIndexer::is_od(res) {
        ODMethod::EyyIndexer
    } else if fancyindex::FancyIndex::is_od(res) {
        ODMethod::FancyIndex
    } else if ab::AB::is_od(res) {
        ODMethod::AB
    } else {
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
    if microsoftiis::MicrosoftIIS::is_od(res, server_name) {
        ODMethod::MicrosoftIIS
    } else if lighttpd::LightTPD::is_od(res, server_name) {
        ODMethod::LightTPD
    } else if apache::Apache::is_od(res, server_name) {
        ODMethod::Apache
    } else if nginx::NGINX::is_od(res, server_name) {
        ODMethod::NGINX
    } else {
        ODMethod::Generic
    }
}
