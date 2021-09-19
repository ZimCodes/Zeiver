use reqwest;

pub mod olaindex;
pub mod autoindex_php;
pub mod directory_lister;
pub mod apache;

#[derive(PartialEq, Debug)]
pub enum ODMethod {
    OLAINDEX,
    AutoIndexPHP,
    AutoIndexPHPNoCrumb,
    DirectoryLister,
    Apache,
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
pub async fn od_type_from_document(res: &str,client:&reqwest::Client,url:&str,tries:u32,wait:Option<f32>,
                                   retry_wait:f32,is_random:bool, verbose:bool) -> Result<ODMethod,reqwest::Error> {
    if olaindex::OLAINDEX::is_od(res) {
        Ok(ODMethod::OLAINDEX)
    } else if directory_lister::DirectoryLister::is_od(res){
        Ok(ODMethod::DirectoryLister)
    }else if apache::Apache::is_od(res,client,url,tries,wait,retry_wait,is_random,verbose).await?{
        Ok(ODMethod::Apache)
    }else {
        let (breadcrumb_exist, is_autoindex) = autoindex_php::AutoIndexPHP::is_od(res);
        if breadcrumb_exist && is_autoindex {
            Ok(ODMethod::AutoIndexPHP)
        }else if is_autoindex {
            Ok(ODMethod::AutoIndexPHPNoCrumb)
        }else {
            Ok(ODMethod::Generic)
        }
    }
}
