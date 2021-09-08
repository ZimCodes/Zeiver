use lazy_static::lazy_static;
use regex::Regex;
use url::Url;

lazy_static!{
    static ref BACK_REG:Regex = Regex::new(r"(?:\.\./)").unwrap();
    static ref REL_FILE_EXT_REG:Regex = Regex::new(r"\.[a-zA-Z0-9][a-zA-Z][a-zA-Z0-9]{1,5}/?$").unwrap();
    static ref URL_FILE_EXT_REG:Regex = Regex::new(r"\w/[a-zA-Z0-9~\+\-%\[\]\$_\.!‘\(\)= ]+\.[a-zA-Z0-9][a-zA-Z][a-zA-Z0-9]{1,5}/?$").unwrap();
    static ref PREVIEW_REG:Regex = Regex::new(r"\?preview$").unwrap();
    static ref SYMBOLS_REG:Regex = Regex::new(r"/?[a-zA-Z0-9\*~\+\-%\?\[\]\$_\.!‘\(\)=]+/").unwrap();
    static ref QUERY_PATH_REG:Regex = Regex::new(r"/\?/").unwrap();
    static ref LAST_SLASH_REG:Regex = Regex::new(r"/$").unwrap();
    static ref DUPLICATE_SLASH_REG:Regex = Regex::new(r"[^:]//\w+").unwrap();
    static ref WEB_REG:Regex = Regex::new(r"[a-zA-Z0-9~\+\-%\[\]\$_\.!‘\(\)=]+\.(html?|aspx?|php)/?$").unwrap();
}
/// Joins the relative & original URL together
/// 1.) If first path of URL matches first path of relative URL,
/// set relative as the new path URL for original URL.
/// 2.) If the relative URL starts with a query,
/// set relative path as the new path URL for original url
/// 3.) If the relative URL starts with a 'dir' query,
/// set relative path as the new path URL for original url
/// 4.) Otherwise add relative URL onto the path of original URL
pub fn url_joiner(url:&str,rel:&str) -> String{
    let url = DUPLICATE_SLASH_REG.replace(url,"/");
    let mut url = no_query_path(url.as_ref());
    let dummy_url = if !rel.starts_with("/"){
        format!("http://www.example.com/{}",rel)
    }else{
        format!("http://www.example.com{}",rel)
    };

    let rel_url = no_query_path(dummy_url.as_str());
    let mut url_path_segments = url.path_segments().expect("Cannot be base");
    let mut rel_path_segments = rel_url.path_segments().expect("cannot be base");

    let url_path = url_path_segments.next().expect("First URL path cannot be found!");
    let rel_path = rel_path_segments.next().expect("First relative URL path cannot be found!");
    if url_path == rel_path && !url_path.is_empty() && !rel_path.is_empty() {
        let scheme = url.scheme();
        let host = url.host().expect("URL does not have a host to be joined with!");
        let path =  rel;
        let path = match path.starts_with("/"){
            true => path.to_string(),
            false => format!("/{}",path)
        };
        match url.port(){
            Some(port) => {
                format!("{}://{}:{}{}",scheme,host,port,path)
            },
            None => {
                format!("{}://{}{}",scheme,host,path)
            }
        }
    }
    else if rel.starts_with("?")
    {
        url.set_query(Some(&rel[1..rel.len()]));
        url.to_string()
    }
    else if url.query().is_some() && url.query().unwrap().starts_with("dir="){
        url.set_query(Some(rel));
        url.to_string().replace(&*format!("?{}",url.query().unwrap()),rel)
    }
    else if WEB_REG.is_match(url.as_str()){
        let url = WEB_REG.replace(url.as_str(),rel);
        url.to_string()
    }
    else{
        if url.as_str().ends_with("/") && rel.starts_with("/"){
            let new_url = remove_last_slash(url.as_str());
            format!("{}{}",new_url,rel)
        }else{
            format!("{}{}",url,rel)
        }


    }
}
/// Checks if the directory query from the URL,'?dir=', matches
/// the relative URL
pub fn check_dir_query(url:&str,rel:&str)-> bool{
    let url = Url::parse(url).unwrap();
    let query = match url.query(){
        Some(query) => query,
        None => ""
    };

    let rel = &rel[1..rel.len()];
    rel.contains(query) && rel != query
}
/// Determines if the URL is a direct link to a file.
/// File must not be an `htm(l),php,asp(x)` file type.
pub fn is_uri(url:&str) -> bool{
    URL_FILE_EXT_REG.is_match(url) && (!WEB_REG.is_match(url))
}
/// Removes the /?/ path from the URL
/// NOTE: Some URLs have a /?/ as the first path. Using URL::path_segment() will not
/// identify it as a path segment. Instead, it is considered a query
fn no_query_path(url:&str) -> Url {
    let url_no_query = QUERY_PATH_REG.replace_all(url,"/");
    let url = Url::parse(&*url_no_query).expect("Cannot parse &str into an URL type");
    url
}
/// Removes the last slash from the URL
pub fn remove_last_slash(url:&str) -> String{
    if url.ends_with("/") {
        let new_url = LAST_SLASH_REG.replace(url,"");
        new_url.to_string()
    }else{
        url.to_string()
    }

}
/// Removes the '?preview' query from an URL
pub fn remove_preview_query(url:&str) -> String{
    if url.ends_with("?preview"){
        PREVIEW_REG.replace(url,"").to_string()
    }else{
        url.to_string()
    }

}
/// Removes the '?preview' query & adds a `/` to the end of the URL
pub fn add_last_slash(url: &str) -> String {
    let mut url = remove_preview_query(url);
    url = add_scheme(url);
    if !url.ends_with("/") {
        url.push('/');
        url
    }else{
        url.to_string()
    }

}
/// Adds the http scheme to a URL
fn add_scheme(url:String) -> String{
    let scheme:&str = "http://";
    if !url.starts_with("http://")
        && !url.starts_with("https://"){
        format!("{}{}",scheme,url)
    }else{
        url
    }

}
/// Checks if relative URL is a symbol
/// # Example:
/// ```首页, 驱动器,시간짜리```
pub fn is_not_symbol(rel_url:&str) -> bool{
    SYMBOLS_REG.is_match(rel_url)
}
/// Checks if URL has a file extension
pub fn is_file_ext(url:&str)->bool{
    REL_FILE_EXT_REG.is_match(url)
}
/// Checks if relative URL matches `../`
pub fn is_back_url(rel_url:&str) -> bool{
    BACK_REG.is_match(rel_url)
}
/// Checks if relative URL is a home path
pub fn is_home_url(rel_url:&str) -> bool{
    rel_url == "/"
}
/// Checks if relative path is the same as original URL path.
/// If there are more relative path segments than the current original URL, then the
/// relative URL is valid.
pub fn is_url_path(url:&str,rel_url:&str) -> bool{
    let url= no_query_path(url);
    let rel_url = no_query_path(&*format!("http://www.example.com{}",rel_url));

    match rel_url.path_segments(){
        Some(rel_path_iter) =>{
            match url.path_segments(){
                Some(url_path_iter) =>{
                    rel_path_iter.count() <= url_path_iter.count()
                },
                None => false
            }
        },
        None => true
    }
}
/// Create a new Regex struct
pub fn set_regex(regex:&Option<String>) -> Regex{
        let regex_pat = regex.as_ref().unwrap();
        Regex::new(&*format!(r"{}",regex_pat)).unwrap()
}
/*Sanitize the url to for easy traversing*/
pub fn sanitize_url(url:&str) ->String{
    use crate::od::olaindex::OLAINDEX;
    let url = OLAINDEX::sanitize_url(url);
    let url = remove_preview_query(url.as_ref());
    String::from(url)
}
/// Check if url is the parent directory of the href link
pub fn sub_dir_check(x:&str,url:&str)->bool{
    if !x.starts_with(url) {
        let mut rel:Vec<&str> = x.split('/').collect();
        let mut new_url:Vec<&str> = url.split('/').collect();

        if rel.len()  < 4 {
            return false;
        }
        rel.remove(3);
        new_url.remove(3);

        rel.join("/").starts_with(&new_url.join("/"))
    }else{
        true
    }
}