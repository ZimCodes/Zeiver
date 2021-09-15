use lazy_static::lazy_static;
use regex::Regex;
use std::borrow::Cow;
use select::document::Document;
use select::predicate::{Name, Attr, Predicate};
lazy_static!{
    static ref OLAINDEX_HASH_QUERY:Regex = Regex::new(r"\?hash=[0-9a-zA-Z]{8}(/?|&download=1)$").unwrap();
    static ref OLAINDEX_QUERIES:Regex = Regex::new(r"\?hash=[0-9a-zA-Z]{8}&download=1$").unwrap();
}
pub enum OlaindexExtras {
    All,
    ExcludeHomeAndDownload
}
pub struct OLAINDEX{}
impl OLAINDEX{
    /// Check to see if url has download query to the OLAINDEX ODs
    /// [Identity] Ex: https://example.com/coolthing.mp4?download=1
    pub fn has_dl_query(x: &str) -> bool{
        x.ends_with("?download=1")
    }
    ///Custom version for files to Accept/Reject (OLAINDEX)
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
    /// Checks for hash query in URL
    /// [Identity]
    pub fn hash_query(x:&str) ->bool{
        OLAINDEX_HASH_QUERY.is_match(x)
    }
    /// Removes hash query
    pub fn sanitize_url(url:&str) ->Cow<str>{
        OLAINDEX_HASH_QUERY.replace(url,"")
    }
    /// Add download query to string
    /// [Transform]
    pub fn add_dl_query(x:&String)-> String{
        format!("{}?download=1",x)
    }
    /// Check if path contains any extra paths such as '/d/,/down/,/v/,/view/,/home/,etc.'
    pub fn has_extra_paths(paths: &mut Vec<&str>,include:OlaindexExtras)->bool{
        let extra_path = paths.get(3);

        match extra_path{
            Some(path)=>{
                let is_common_search =  path == &"v" || path == &"view";
                let is_down_search = path == &"d" || path == &"down";
                let is_show_search = path == &"s" || path == &"show";
                let is_home = path == &"home";
                match include {
                    OlaindexExtras::ExcludeHomeAndDownload => is_common_search || is_show_search,
                    _ =>  is_common_search ||  is_down_search || is_show_search || is_home
                }
            },
            None => false
        }
    }
    /// Removes extra paths from broken down url
    pub fn remove_extra_paths(paths: &mut Vec<&str>, include:OlaindexExtras){
        if OLAINDEX::has_extra_paths(paths, include){
            paths.remove(3);
        }
    }
    /// Check if data-route attribute exists in Document.
    /// [Type:] Used to determine od type
    pub fn has_data_route(res:&str)-> bool {
        Document::from(res)
            .find(Name("a").and(Attr("data-route", ())))
            .any(|x| x == x)
    }
}

