pub struct Apache;
use http::Http;
use reqwest;
use regex::Regex;
use lazy_static::lazy_static;
use select::document::Document;
use select::predicate::Name;

const IDENTIFIER:&str = "Apache";

lazy_static!{
    static ref SORT_QUERIES:Regex = Regex::new(r"\?[A-Z]=[A-Z](;[A-Z]=[A-Z])?$").unwrap();
}
impl Apache{
    pub async fn is_od(res:&str,client:&reqwest::Client,url:&str,tries:u32,wait:Option<f32>,retry_wait:f32,is_random:bool,
                 verbose:bool)-> Result<bool,reqwest::Error>{
        let response = Http::get_response(client,url,tries,wait,retry_wait,is_random,verbose).await?;
         let is_od = match response.headers().get("server"){
             Some(server) => {
                 let server_name = server.to_str().unwrap();
                 server_name.contains(IDENTIFIER)
             },
             None => false
         };
        if !is_od{
            Ok(Apache::address_check(res))
        }else{
            Ok(is_od)
        }
    }
    /// Check for id in the address tag
    fn address_check(res:&str)-> bool{
        Document::from(res).find(Name("address"))
            .any(|node| node.text().contains(IDENTIFIER))
    }
    pub fn has_extra_query(x:&str)->bool{
        SORT_QUERIES.is_match(x)
    }
}