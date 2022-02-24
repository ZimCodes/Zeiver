use crate::od::ab::AB;
use crate::od::all;
use crate::od::apache::Apache;
use crate::od::apache_directory_listing::ApacheDirectoryListing;
use crate::od::autoindex_php::AutoIndexPHP;
use crate::od::directory_lister::DirectoryLister;
use crate::od::directory_listing_script::DirectoryListingScript;
use crate::od::eyy_indexer::EyyIndexer;
use crate::od::fancyindex::FancyIndex;
use crate::od::h5ai::H5AI;
use crate::od::lighttpd::LightTPD;
use crate::od::microsoftiis::MicrosoftIIS;
use crate::od::odindex::OdIndex;
use crate::od::olaindex::OLAINDEX;
use crate::od::oneindex::OneIndex;
use crate::od::onemanager::OneManager;
use crate::od::panindex::PanIndex;
use crate::od::phpbb::PHPBB;
use crate::od::snif::Snif;
use crate::od::windex::Windex;
use crate::od::ODMethod;

/// Switch to a different way to parse Document type
pub fn filtered_links(res: &str, url: &str, od_type: &ODMethod) -> Vec<String> {
    match od_type {
        ODMethod::OLAINDEX => OLAINDEX::search(res, url),
        ODMethod::AutoIndexPHP | ODMethod::AutoIndexPHPNoCrumb => AutoIndexPHP::search(res, url),
        ODMethod::DirectoryLister => DirectoryLister::search(res, url),
        ODMethod::DirectoryListingScript => DirectoryListingScript::search(res, url),
        ODMethod::PHPBB => PHPBB::search(res, url),
        ODMethod::OneIndex => OneIndex::search(res, url),
        ODMethod::OneManager => OneManager::search(res, url),
        ODMethod::H5AI => H5AI::search(res, url),
        ODMethod::MicrosoftIIS => MicrosoftIIS::search(res, url),
        ODMethod::Snif => Snif::search(res, url),
        ODMethod::PanIndex => PanIndex::search(res, url),
        ODMethod::OdIndex => OdIndex::search(res, url),
        ODMethod::Windex => Windex::search(res, url),
        ODMethod::ApacheDirectoryListing => ApacheDirectoryListing::search(res, url),
        ODMethod::EyyIndexer => EyyIndexer::search(res, url),
        ODMethod::FancyIndex => FancyIndex::search(res, url),
        ODMethod::AB => AB::search(res, url),
        ODMethod::LightTPD => LightTPD::search(res, url),
        ODMethod::Apache | ODMethod::NGINX => Apache::search(res, url),
        _ => all::search(res, url),
    }
}
