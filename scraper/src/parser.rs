use lazy_static::lazy_static;
use regex::Regex;
use url::Url;
use crate::od::olaindex::{OLAINDEX, OlaindexExtras};

lazy_static! {
    static ref BACK_REG:Regex = Regex::new(r"(?:\.\./)").unwrap();
    static ref REL_FILE_EXT_REG:Regex = Regex::new(r"\.(?:[a-zA-Z0-9]{3,7}|[a-zA-Z][a-zA-Z0-9]|[0-9][a-zA-Z])/?$").unwrap();
    static ref URL_FILE_EXT_REG:Regex = Regex::new(r"\w/[a-zA-Z0-9~\+\-%\[\]\$_\.!‘\(\)= ]+\.(?:[a-zA-Z0-9]{3,7}|[a-zA-Z][a-zA-Z0-9]|[0-9][a-zA-Z])/?$").unwrap();
    static ref PREVIEW_REG:Regex = Regex::new(r"\?preview$").unwrap();
    static ref SYMBOLS_REG:Regex = Regex::new(r"/?[a-zA-Z0-9\*~\+\-%\?\[\]\$_\.!‘\(\)=]+/").unwrap();
    static ref QUERY_PATH_REG:Regex = Regex::new(r"/(\?|\.)/").unwrap();
    static ref LAST_SLASH_REG:Regex = Regex::new(r"/$").unwrap();
    static ref DUPLICATE_SLASH_REG:Regex = Regex::new(r"[^:]//\w+").unwrap();
    static ref WEB_REG:Regex = Regex::new(r"[a-zA-Z0-9~\+\-%\[\]\$_\.!‘\(\)=]+\.(html?|aspx?|php)/?$").unwrap();
    static ref PAGE_QUERY_REG:Regex = Regex::new(r"\?page=([0-9]{1,3})$").unwrap();
    static ref DIR_QUERIES_REG:Regex = Regex::new(r"((/[a-zA-Z]+\.php/?\?dir=)|(/?\?dir=))(\.(/|%2F))?").unwrap();
    static ref WWW_REG:Regex = Regex::new(r"www\.").unwrap();
    static ref HTTP_REG:Regex = Regex::new(r"^https?://").unwrap();
}
/// Joins the relative & original URL together
/// 1.) If first path of URL matches first path of relative URL,
/// set relative as the new path URL for original URL.
/// 2.) If the relative URL starts with a query,
/// set relative path as the new path URL for original url
/// 3.) If the relative URL starts with a 'dir' query,
/// set relative path as the new path URL for original url
/// 4.) Otherwise add relative URL onto the path of original URL
pub fn url_joiner(url: &str, rel: &str) -> String {
    let url = DUPLICATE_SLASH_REG.replace(url, "/");
    let mut url = no_query_path(url.as_ref());
    let dummy_url = if rel.starts_with("./") {
        format!("http://www.example.com{}", &rel[2..])
    } else if !rel.starts_with("/"){
        format!("http://www.example.com/{}", rel)
    } else {
        format!("http://www.example.com{}", rel)
    };

    let rel_url = no_query_path(dummy_url.as_str());
    let mut url_path_segments = url.path_segments().expect("Cannot be base");
    let mut rel_path_segments = rel_url.path_segments().expect("cannot be base");

    let url_path = url_path_segments.next().expect("First URL path cannot be found!");
    let rel_path = rel_path_segments.next().expect("First relative URL path cannot be found!");
    if url_path == rel_path && !url_path.is_empty() && !rel_path.is_empty() {
        let scheme = url.scheme();
        let host = url.host().expect("URL does not have a host to be joined with!");
        let path = rel;
        let path = match path.starts_with("/") {
            true => path.to_string(),
            false => format!("/{}", path)
        };
        match url.port() {
            Some(port) => format!("{}://{}:{}{}", scheme, host, port, path),
            None => format!("{}://{}{}", scheme, host, path)
        }
    } else if rel.starts_with("?")
    {
        url.set_query(Some(&rel[1..rel.len()]));
        url.to_string()
    } else if url.query().is_some() && url.query().unwrap().starts_with("dir=") {
        url.set_query(Some(rel));
        let url_query = url.query().unwrap();
        let text_replace = if url.to_string().contains("index.php?"){
            format!("index.php?{}",url_query)
        }else{
            format!("?{}",url_query)
        };

        url.to_string().replace(text_replace.as_str(), if rel.starts_with("/"){
            &rel[1..]
        }else{
            rel
        })
    } else if WEB_REG.is_match(url.as_str()) {
        if rel.starts_with("./") {
            let url = WEB_REG.replace(url.as_str(), &rel[2..]);
            url.to_string()
        }else {
            let url = WEB_REG.replace(url.as_str(), rel);
            url.to_string()
        }
    } else {
        if url.as_str().ends_with("/") && rel.starts_with("/") {
            let new_url = remove_last_slash(url.as_str());
            format!("{}{}", new_url, rel)
        } else if rel.starts_with("./") && url.as_str().ends_with("/"){
            format!("{}{}",url,&rel[2..])
        } else if rel.starts_with("./") && !url.as_str().ends_with("/"){
            format!("{}{}",url,&rel[1..])
        }
        else {
            format!("{}{}", url, rel)
        }
    }
}

/// Checks if the directory query from the URL,'?dir=', matches
/// the relative URL
pub fn check_dir_query(url: &str, rel: &str) -> bool {
    let has_dir_query = has_dir_queries(rel);
    let url = Url::parse(url).unwrap();
    let query = match url.query() {
        Some(query) => query,
        None => ""
    };
    let rel = &rel[1..rel.len()];
    if query.ends_with("/"){
        let query_encoded_slash = query.replace("/","%2F");
        rel.contains(query_encoded_slash.as_str()) && rel != query && has_dir_query
    }else{
        rel.contains(query) && rel != query && has_dir_query
    }
}
/// Check for navigator queries
fn has_dir_queries(rel:&str)-> bool{
    DIR_QUERIES_REG.is_match(rel)
}
/// Replace navigator query
fn replace_dir_queries(x:&str,repl:&str) -> String{
    DIR_QUERIES_REG.replace(x,repl).to_string()
}
/// Determines if the URL is a direct link to a file.
/// File must not be an `htm(l),php,asp(x)` file type.
pub fn is_uri(url: &str) -> bool {
    URL_FILE_EXT_REG.is_match(url) && (!WEB_REG.is_match(url))
}

/// Removes the /?/ path from the URL
/// NOTE: Some URLs have a /?/ as the first path. Using URL::path_segment() will not
/// identify it as a path segment. Instead, it is considered a query
fn no_query_path(url: &str) -> Url {
    let url_no_query = removes_single_path(url);
    let url = Url::parse(&*url_no_query).expect("Cannot parse &str into an URL type");
    url
}
/// Removes the /?/ & /./ paths from the URL
pub fn removes_single_path(path:&str)-> String{
    QUERY_PATH_REG.replace_all(path, "/").to_string()
}
/// Removes the last slash from the URL
pub fn remove_last_slash(url: &str) -> String {
    if url.ends_with("/") {
        let new_url = LAST_SLASH_REG.replace(url, "");
        new_url.to_string()
    } else {
        url.to_string()
    }
}
/// Removes HTTP scheme from string
pub fn remove_http(path:&str) ->String{
    HTTP_REG.replace(path,"").to_string()
}
/// Checks if url has an authentic HTTP scheme
pub fn is_http(url:&str)->bool{
    HTTP_REG.is_match(url)
}
/// Removes the '?preview' query from an URL
pub fn remove_preview_query(url: &str) -> String {
    if url.ends_with("?preview") {
        PREVIEW_REG.replace(url, "").to_string()
    } else {
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
    } else {
        url.to_string()
    }
}

/// Adds the http scheme to a URL
fn add_scheme(url: String) -> String {
    let scheme: &str = "http://";
    if !url.starts_with("http://")
        && !url.starts_with("https://") {
        format!("{}{}", scheme, url)
    } else {
        url
    }
}
/// Checks if relative URL is a symbol
/// # Example:
/// ```首页, 驱动器,시간짜리```
pub fn is_not_symbol(rel_url: &str) -> bool {
    SYMBOLS_REG.is_match(rel_url)
}

/// Checks if URL has a file extension
pub fn is_file_ext(url: &str) -> bool {
    REL_FILE_EXT_REG.is_match(url)
}

/// Checks if relative URL matches `../`
pub fn is_back_url(rel_url: &str) -> bool {
    BACK_REG.is_match(rel_url)
}

/// Checks if relative URL is a home path
pub fn is_home_url(rel_url: &str) -> bool {
    rel_url == "/"
}
/// Create a new Regex struct
pub fn set_regex(regex: &Option<String>) -> Regex {
    let regex_pat = regex.as_ref().unwrap();
    Regex::new(&*format!(r"{}", regex_pat)).unwrap()
}
///Sanitize the url to for easy traversing
pub fn sanitize_url(url: &str) -> String {
    let url = OLAINDEX::sanitize_url(url);
    let url = remove_preview_query(url.as_ref());
    let url = remove_space_entity(&url);
    String::from(url)
}
///Remove `%20` space HTML Encode at the end of link
fn remove_space_entity(url:&str)-> &str{
    if url.ends_with("%20"){
        &url[..url.len()-3]
    }else{
        url
    }
}
///HTML Encode then compare url with relative path
pub fn encode_slash_starts_with(rel:&str,url:&str)->bool{
    let url = ready_url_for_checking(url,"/");
    let rel = ready_url_for_checking(rel,"/");
    rel.starts_with(&url)
}
/// Lightly HTML encode a string of text
fn html_encode(txt:&str)-> String{
    let mut txt = txt.replace("/","%2F");
    txt = txt.replace("+","%20");
    txt = txt.replace(" ","%20");
    txt
}
/// Modify the url in order for it to be valid for checking
fn ready_url_for_checking(x:&str,repl:&str)-> String{
    let mut x = remove_last_slash(x);
    x = replace_dir_queries(&x,repl);
    x = WWW_REG.replace(&x,"").to_string();// Remove `www.` from path
    x = OLAINDEX::remove_path(&x);
    x = html_encode(&x);
    x
}
/// Check if url is the parent directory of the href link
pub fn sub_dir_check(x: &str, url: &str) -> bool {
    let x = ready_url_for_checking(x,"%2F");
    let url = ready_url_for_checking(url,"%2F");
    if !x.starts_with(&url) {
        let mut rel: Vec<&str> = x.split('/').collect();
        let mut new_url: Vec<&str> = url.split('/').collect();

        //The root of the URL Ex: domain.com
        if rel.len() < 4 {
            return false;
        }

        OLAINDEX::remove_extra_paths(&mut rel, OlaindexExtras::All);
        OLAINDEX::remove_extra_paths(&mut new_url, OlaindexExtras::All);
        let new_url = new_url.join("/");
        let new_url = PAGE_QUERY_REG.replace(&*new_url, "");

        rel.join("/").starts_with(&new_url.as_ref())
    } else {
        true
    }
}
/// Checks if the path has a page query
pub fn has_page_query(rel: &str) -> bool{
    PAGE_QUERY_REG.is_match(rel)
}
/// Keeps url with `?page=` from being traversed backwards
#[allow(unused_assignments)]
pub fn within_page_limit(rel:&str,mut current_page:usize)-> bool{
    if !has_page_query(rel){
        return false;
    }
    let page_num = match PAGE_QUERY_REG.captures(rel){
        Some(captures) => captures.get(1).unwrap().as_str().parse::<usize>().unwrap(),
        None => 0
    };

    if current_page < page_num {
        current_page += 1;
        true
    }else{
        false
    }
}
/// Case-Insensitive `str.contains()` variant
fn ins_contains(rel: &str, text: &str) -> bool {
    rel.to_lowercase().contains(text)
}
/// Queries that is used not need for traversing a directory
pub fn unrelated_dir_queries(rel: &str) -> bool {
    ins_contains(rel, "sortby")
        || ins_contains(rel, "&sort_mode=") || ins_contains(rel, "&sort=")
        || ins_contains(rel, "&file=") || ins_contains(rel, "archive=true")
        || ins_contains(rel, "&expand=") || ins_contains(rel, "&collapse=")
}

/// Check if URL is the same as relative path
/// in order to prevent traversing the home directory twice.
/// Ex: https://example.com/index.php == /index.php -> true
pub fn is_rel_url(url: &str, rel: &str) -> bool {
    if !rel.starts_with("/") {
        url == rel
    } else {
        if url.ends_with("/") {
            &url[..url.len() - 1] == url_joiner(url, rel).as_str()
        } else {
            url == url_joiner(url, rel).as_str()
        }
    }
}


#[cfg(test)]
mod tests{
    use super::{is_file_ext,has_dir_queries,QUERY_PATH_REG};
    #[test]
    fn file_regex_test(){
        assert!(is_file_ext("Example.3gp"));//start num
        assert!(is_file_ext("Example.7z.001"));//Length at 3
        assert!(!is_file_ext("Example.7z.01"));//Length below 3
        assert!(is_file_ext("Example.u3i"));//letter #
        assert!(is_file_ext("Example.PWD"));//caps
        assert!(is_file_ext("Example.60D"));//2 #s starting
        assert!(!is_file_ext("Example.87fesf27"));//Length above 7
        assert!(is_file_ext("Example.H13"));//letter ##
        assert!(is_file_ext("Example.alpx"));//all letters
    }
    #[test]
    fn dir_query_regex(){
        assert_eq!(has_dir_queries("/index.php?dir=Hello+World%2F%2Fthumbnails%2F"),true);
        assert_eq!(has_dir_queries("/file.php?dir=Hello+World%2F%2Fthumbnails%2F"),true);
        assert_eq!(has_dir_queries("help.php?dir=Hello+World%2F%2Fthumbnails%2F"),true);
        assert_eq!(has_dir_queries("?dir=Hello+World%2F%2Fthumbnails%2F"),true);
        assert_eq!(has_dir_queries("/?dir=Hello+World%2F%2Fthumbnails%2F"),true);
        assert_eq!(has_dir_queries("/?dir=./Hello+World%2F%2Fthumbnails%2F"),true);
        assert_eq!(has_dir_queries("/?dir=.%2FHello+World%2F%2Fthumbnails%2F"),true);
        assert_eq!(has_dir_queries("/?dir=.Hello+World%2F%2Fthumbnails%2F"),true);
        assert_eq!(has_dir_queries("/?dir=/Hello+World%2F%2Fthumbnails%2F"),true);

        assert_eq!(has_dir_queries("/dir=Hello+World%2F%2Fthumbnails%2F"),false);
        assert_eq!(has_dir_queries("dir=Hello+World%2F%2Fthumbnails%2F"),false);
    }
    #[test]
    fn one_non_letter_path(){
        assert_eq!(QUERY_PATH_REG.is_match("http://cool.example.net/./ChampionHat/This_Battery_Pack_tut.pdf"),true);
        assert_eq!(QUERY_PATH_REG.is_match("http://cool.example.net/?/ChampionHat/This_Battery_Pack_tut.pdf"),true);
        assert_eq!(QUERY_PATH_REG.is_match("http://cool.example.net/?/./ChampionHat/This_Battery_Pack_tut.pdf"),true);
        assert_eq!(QUERY_PATH_REG.is_match("http://cool.example.net/././ChampionHat/This_Battery_Pack_tut.pdf"),true);
        assert_eq!(QUERY_PATH_REG.is_match("http://cool.example.net/?/./ChampionHat/This_Battery_Pack_tut.pdf"),true);
        assert_eq!(QUERY_PATH_REG.is_match("http://cool.example.net/?/?/ChampionHat/This_Battery_Pack_tut.pdf"),true);

        assert_eq!(QUERY_PATH_REG.is_match("http://cool.example.net/.ChampionHat/This_Battery_Pack_tut.pdf"),false);
        assert_eq!(QUERY_PATH_REG.is_match("http://cool.example.net/Champion?Hat/This?_Battery_Pack_tut.pdf"),false);
        assert_eq!(QUERY_PATH_REG.is_match("http://cool.example.net/?ChampionHat/This_Battery_Pack_tut.pdf"),false);
    }
}