use tokio::fs;
use tokio::io::ErrorKind;
use std::path::PathBuf;
use reqwest;
use asset;
use http;
mod parser;
mod od;
mod search;

pub struct Scraper{
    pub pages:Vec<asset::page::Page>,
    dir_links:Vec<String>,
    od_type:Option<String>
}
impl Scraper{
    pub fn new() -> Scraper{
        let pages = Vec::new();
        let dir_links = Vec::new();
        let od_type = None;
        Scraper{
            pages,
            dir_links,
            od_type
        }
    }
    /// Scrape files URLs present on the current page(URL)
    fn scrape_files(&mut self,res:&str,url:&str,accept:&Option<String>,reject:&Option<String>,verbose:bool)
                    -> Vec<asset::file::File>
    {
        let mut files:Vec<asset::file::File> = Vec::new();
        let mut previous_file = String::new();//variable to check for duplicates

        println!("-----Parsing File Links-----");
        search::filtered_links(res)
            .iter()
            .for_each(|x|{
                if &previous_file != x {
                    previous_file = x.to_string();
                    let is_file_ext = parser::is_file_ext(x.as_str());

                    let ending_check = is_file_ext
                        || od::olaindex::OLAINDEX::has_dl_query(&x)
                        || od::olaindex::OLAINDEX::hash_query(&x);

                    let sub_check = parser::sub_dir_check(&x, url);

                    if  ending_check
                        && !x.ends_with("/")
                        && (!x.starts_with("http")
                        || sub_check)
                    {
                        if !x.starts_with("?dir=")
                            || (x.starts_with("?dir=") && parser::check_dir_query(url, x.as_str()))
                        {
                            let mut x = String::from(x);
                            if self.od_type.is_some() && self.od_type.as_ref().unwrap() == "olaindex"{
                                x = od::olaindex::OLAINDEX::add_dl_query(&x);
                            }

                            if od::olaindex::OLAINDEX::has_dl_query(&x) {
                                let (new_accept,new_reject) = od::olaindex::OLAINDEX::acc_rej_filters(&accept, &reject);
                                Scraper::acc_rej_check(url, &mut files, &x, &new_accept, &new_reject, verbose);
                            }else{
                                Scraper::acc_rej_check(url, &mut files, &x, accept, reject, verbose);
                            }
                        }
                    }
                }
            });
        println!("-----End of Parsing File Links-----");
        files
    }
    /// Scrape directory URLs present on the current page(URL)
    fn scrape_dirs(&mut self,res:&str, url:&str, verbose:bool) -> Vec<asset::directory::Directory>{
        let mut dirs = Vec::new();
        let mut past_dir = String::new();//variable to check for duplicates

        println!("-----Parsing Directory Links-----");
        search::filtered_links(res)
            .iter()
            .for_each(|x|{

            let x = &*x;
            let current_dir = format!("{}{}",url,x);
            if past_dir != current_dir {

                past_dir = current_dir;

                if  (!x.starts_with("http")
                    || x.starts_with(url))
                    && !parser::is_back_url(x)
                    && !parser::is_home_url(x)
                    && (!x.contains("sortBy") && !x.contains("sortby"))
                {
                    if  parser::is_not_symbol(x)
                        && ((x.starts_with("/")
                        && !parser::is_url_path(url, x))
                        || !x.starts_with("/")
                        && !x.starts_with("?"))
                        && !od::olaindex::OLAINDEX::has_dl_query(&x)
                        && !parser::is_file_ext(x)
                    {
                        self.add_dir(url,x,&mut dirs,verbose);
                    }
                    else if x.starts_with("?dir=") && parser::check_dir_query(url, x){
                        self.add_dir(url,x,&mut dirs,verbose);
                    }
                }
            }
        });
        println!("-----End of Parsing Directory Links-----");
        dirs
    }
    pub async fn run(&mut self,client:&reqwest::Client,url:&str,accept:&Option<String>,
                     reject:&Option<String>,depth:usize,tries:u32,wait:Option<f32>,
                     retry_wait:f32,is_random:bool,verbose:bool)
                     ->Result<(),reqwest::Error>
    {
        let url_string = parser::add_last_slash(url);
        let url = url_string.as_str();

        //Check if URL points to a file
        if self.single_scrape(url,verbose){
            return Ok(());
        }
        self.retrieve_od_type(url);

        let url = parser::sanitize_url(url);

        //Retrieve page
        let res = http::Http::connect(client,&url,tries,wait,retry_wait,is_random,verbose).await?;

        let dirs_of_dirs = vec![self.scrape_dirs(res.as_str(),&url,verbose)];

        let files = self.scrape_files(res.as_str(), &url,&accept,&reject,verbose);
        if !files.is_empty(){
            self.pages.push(asset::page::Page::new(files));
        }

        //Determines whether to start recursive scraping
        let is_dir_empty = dirs_of_dirs.get(0).unwrap().is_empty();
        if !is_dir_empty{
            self.dir_recursive(client,&url,res,dirs_of_dirs,accept,reject,depth,tries,wait,retry_wait,is_random,verbose).await?;
        }

        Ok(())
    }
    /// Adds a URL to the list of directories cycle through
    fn add_dir(&mut self,url:&str,x:&str,dirs:&mut Vec<asset::directory::Directory>,verbose:bool){
        let joined_url =  if x.starts_with("http") {
            String::from(x)
        } else{
            parser::url_joiner(url, x)
        };
        if verbose {
            println!("DIR: {}",joined_url);
        }
        if !self.dir_links.contains(&joined_url){
            self.dir_links.push(joined_url.clone());
            dirs.push(
                asset::directory::Directory::new(
                    format!("{}",joined_url)
                )
            );
        }
    }
    /// Adds a URL to the list of files to download
    fn add_file(url:&str,x:&str,files:&mut Vec<asset::file::File>,verbose:bool){
        let joined_url =  if x.starts_with("http") {
            String::from(x)
        } else{
            parser::url_joiner(url, x)
        };
        if verbose {
            println!("URI: {}",joined_url);
        }
        files.push(
            asset::file::File::new(
                joined_url.as_str()
            ));
    }
    fn retrieve_od_type(&mut self,url:&str){
        if od::olaindex::OLAINDEX::hash_query(url){
            self.od_type = Some(String::from("olaindex"));
        }
    }
    /// Recursively scrape file URLs from child directories
    /// NOTE: variables ARE being used
    #[allow(unused_assignments)]
    async fn dir_recursive(&mut self,client:&reqwest::Client,mut url:&str,mut res:String, mut dirs_of_dirs:Vec<Vec<asset::directory::Directory>>,
                           accept:&Option<String>, reject:&Option<String>,depth:usize,tries:u32,wait:Option<f32>,retry_wait:f32,is_random:bool,verbose:bool)->Result<(),reqwest::Error>
    {
        println!("-----Starting Directory Diving-----");

        let mut cur_depth = 1;
        while cur_depth < depth {
            let mut new_dirs = Vec::new();
            for dirs in dirs_of_dirs{

                println!("-----Checking next set of Directories-----");

                for dir in dirs {

                    println!("\n-----Current Parsing Directory-----\n{:?}",dir);

                    //Connect to Directory link
                    url = dir.link.as_str();

                    res = http::Http::connect(client,url,tries,wait,retry_wait,is_random,verbose).await?;
                    //Retrieve Files from Directory Link
                    let files = self.scrape_files(res.as_str(),url,&accept,&reject,verbose);
                    if !files.is_empty(){
                        self.pages.push(asset::page::Page::new(files));
                    }

                    //Retrieve Directories from current Directory Link
                    let cur_dirs = self.scrape_dirs(res.as_str(),url,verbose);
                    if !cur_dirs.is_empty(){
                        new_dirs.push(cur_dirs);
                    }
                }

                println!("-----Finished Checking this set of Directories-----");
            }

            // Check if any Directories were found inside any of the previous Directory Links
            // If there aren't any new Directories, stop checking directories
            if !new_dirs.is_empty(){

                println!("-----Setting up new Directories to check-----");

                dirs_of_dirs = new_dirs;
                cur_depth += 1;
            }else{
                println!("-----Finished Directory Diving-----");
                break;
            }
        }
        Ok(())
    }
    /// Scrape the URL that points to a single File.
    fn single_scrape(&mut self,url:&str,verbose:bool) -> bool{
        if parser::is_uri(url){
            if verbose{
                println!("URI: {}",url);
            }

            let pages= vec![
                asset::page::Page::new(
                    vec![asset::file::File::new(parser::remove_last_slash(url).as_str())]
                )
            ];
            self.pages = pages;
            true
        }else{
            false
        }
    }
    /// Scrape files based on 'accept' and 'reject' option configurations
    fn acc_rej_check(url:&str, files:&mut Vec<asset::file::File>,x:&String, accept:&Option<String>, reject:&Option<String>, verbose:bool){
        //Accept Option
        if accept.is_some(){
            let reg = parser::set_regex(accept);
            if reg.is_match(x.as_str()){
                Scraper::add_file(url,x.as_str(),files,verbose);
            }
        }//Reject Option
        else if reject.is_some(){
            let reg = parser::set_regex(reject);
            if !reg.is_match(x.as_str()){
                Scraper::add_file(url,x.as_str(),files,verbose);
            }
        }//Neither Option
        else if accept.is_none() && reject.is_none(){
            Scraper::add_file(url,x.as_str(),files,verbose);
        }
    }
    /// Read links from a file & start downloading
    pub async fn links_from_file(path:&str) -> Vec<PathBuf> {
        let f = fs::read_to_string(path).await;
        let msg =  match f {
            Ok(msg) => msg,
            Err(e)=> match e.kind(){
                ErrorKind::NotFound => panic!("File cannot be found!"),
                ErrorKind::InvalidData => panic!("The contents of the file are not valid UTF-8"),
                _=>{
                    panic!("Error retrieving data from file")
                }
            }
        };
        let links = if cfg!(target_os = "windows"){
            msg.split("\r\n")
        }else{
            msg.split("\n")
        };
        let mut link_strings = Vec::new();
        for link in links{
            link_strings.push(PathBuf::from(link))
        }
        link_strings
    }
}
