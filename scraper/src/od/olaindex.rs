use lazy_static::lazy_static;
use regex::Regex;
use std::borrow::Cow;
lazy_static!{
    static ref OLAINDEX_HASH_QUERY:Regex = Regex::new(r"\?hash=[0-9a-zA-Z]{8}(/?|&download=1)$").unwrap();
    static ref OLAINDEX_QUERIES:Regex = Regex::new(r"\?hash=[0-9a-zA-Z]{8}&download=1$").unwrap();
}
pub struct OLAINDEX{}
impl OLAINDEX{
    /*Check to see if url has download query to the OLAINDEX ODs
    *Ex: https://example.com/coolthing.mp4?download=1 */
    pub fn has_dl_query(x: &str) -> bool{
        x.ends_with("?download=1")
    }
    /*Custom version for files to Accept/Reject (OLAINDEX)*/
    pub fn acc_rej_filters(accept:&Option<String>, reject:&Option<String>) -> (Option<String>, Option<String>){
        let new_accept = if accept.is_some() {
            Some(format!(r"{}(\?download=1$)", accept.as_ref().unwrap()))
        }else{
            None
        };
        let new_reject = if reject.is_some() {
            Some(format!(r"{}(\?download=1$)", reject.as_ref().unwrap()))
        }else{
            None
        };
        (new_accept,new_reject)
    }
    /*Checks for hash query in URL*/
    pub fn hash_query(x:&str) ->bool{
        OLAINDEX_HASH_QUERY.is_match(x)
    }
    /*Removes hash query*/
    pub fn sanitize_url(url:&str) ->Cow<str>{
        OLAINDEX_HASH_QUERY.replace(url,"")
    }
    /*Add download query to string*/
    pub fn add_dl_query(x:&String)-> String{
        format!("{}?download=1",x)
    }
}

