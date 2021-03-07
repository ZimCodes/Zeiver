use select::document::Document;
use select::predicate::Name;
use std::fs;
use std::io::ErrorKind;
use std::path::PathBuf;
use crate::crawler::{asset,http};

mod util;

pub struct Scraper{
    pub pages:Vec<asset::Page>,
}
impl Scraper{
    pub fn new() -> Scraper{
        let pages = Vec::new();

        Scraper{
            pages
        }
    }
    /// Scrape files URLs present on the current page(URL)
    fn scrape_files(res:&str,url:&str,accept:&Option<String>,reject:&Option<String>,verbose:bool)
        -> Vec<asset::File>
    {
        let mut files:Vec<asset::File> = Vec::new();
        let mut previous_file = String::new();//variable to check for duplicates
        if verbose {
            println!("-----Parsing File Links-----");
        }

        Document::from(res)
            .find(Name("a"))
            .filter_map(|n| n.attr("href"))
            .for_each(|x|{
                let x = util::remove_preview_query(x);
                if previous_file != x {

                    previous_file = x.to_string();
                    if !x.starts_with("http")
                       && util::is_file_ext(x.as_str())
                        && !x.ends_with("/")
                    {
                        if !x.starts_with("?dir=")
                            || (x.starts_with("?dir=") && util::check_dir_query(url,x.as_str()))
                        {
                            if accept.is_some(){
                                let reg = util::acc_rej_regex(accept);
                                if reg.is_match(x.as_str()){
                                    Scraper::add_file(url,x.as_str(),&mut files,verbose);
                                }
                            }
                            else if reject.is_some(){
                                let reg = util::acc_rej_regex(reject);
                                if !reg.is_match(x.as_str()){
                                    Scraper::add_file(url,x.as_str(),&mut files,verbose);
                                }
                            }
                            else if accept.is_none() && reject.is_none(){
                                Scraper::add_file(url,x.as_str(),&mut files,verbose);
                            }
                        }

                    }
                }


            });
        files
    }
    /// Scrape directory URLs present on the current page(URL)
    fn scrape_dirs(res:&str, url:&str, verbose:bool) -> Vec<asset::Directory>{
        let mut dirs = Vec::new();
        let mut past_dir = String::new();//variable to check for duplicates

        if verbose {
            println!("-----Parsing Directory Links-----");
        }

        Document::from(res)
            .find(Name("a"))
            .filter_map(|n| n.attr("href"))
            .for_each(|x|{
                let current_dir = format!("{}{}",url,x);
                if past_dir != current_dir {

                    past_dir = current_dir;

                    if  !util::is_back_url(x)
                        && !util::is_home_url(x)
                        && !x.starts_with("http")

                    {

                        if x.ends_with("/")
                            && util::is_not_symbol(x)
                            && ((x.starts_with("/")
                            && !util::is_url_path(url,x))
                            || !x.starts_with("/")
                            && !x.starts_with("?"))
                        {
                            Scraper::add_dir(url,x,&mut dirs,verbose);
                        }
                        else if x.starts_with("?dir=") && util::check_dir_query(url,x){
                            Scraper::add_dir(url,x,&mut dirs,verbose);
                        }
                    }
                }
            });
        dirs
    }
    /// Adds a URL to the list of directories cycle through
    fn add_dir(url:&str,x:&str,dirs:&mut Vec<asset::Directory>,verbose:bool){
        if verbose {
            println!("DIR: {}",util::url_joiner(url,x));
        }
        dirs.push(
            asset::Directory::new(
                format!("{}",util::url_joiner(url,x))
            )
        );
    }
    /// Adds a URL to the list of files to download
    fn add_file(url:&str,x:&str,files:&mut Vec<asset::File>,verbose:bool){
        let joined_url =  util::url_joiner(url,x);
        if verbose {
            println!("URI: {}",joined_url);
        }
        files.push(
            asset::File::new(
                joined_url.as_str()
            ));
    }
    #[tokio::main]
    pub async fn run(&mut self,client:&reqwest::Client,url:&str,accept:&Option<String>,
                     reject:&Option<String>,depth:usize,tries:u32,wait:Option<f32>,
                     retry_wait:f32,is_random:bool,verbose:bool)
        ->Result<(),reqwest::Error>
    {
        let url_string = util::add_last_slash(url);
        let url = url_string.as_str();

        //Check if URL points to a file
        if Scraper::single_scrape(self,url,verbose){
            return Ok(());
        }
        //Init
        let res = http::Http::connect(client,url,tries,wait,retry_wait,is_random,verbose).await?;

        let dirs_of_dirs = vec![Scraper::scrape_dirs(res.as_str(),url,verbose)];

        let files = Scraper::scrape_files(res.as_str(), url,&accept,&reject,verbose);
        if !files.is_empty(){
            self.pages.push(asset::Page::new(files));
        }

        //Determines whether to start recursive scraping
        let is_dir_empty = dirs_of_dirs.get(0).unwrap().is_empty();
        if !is_dir_empty{
            self.dir_recursive(client,url,res,dirs_of_dirs,accept,reject,depth,tries,wait,retry_wait,is_random,verbose).await?;
        }

        Ok(())
    }
    /// Recursively scrape file URLs from child directories
    /// NOTE: variables ARE being used
    #[allow(unused_assignments)]
    async fn dir_recursive(&mut self,client:&reqwest::Client,mut url:&str,mut res:String, mut dirs_of_dirs:Vec<Vec<asset::Directory>>,
                     accept:&Option<String>, reject:&Option<String>,depth:usize,tries:u32,wait:Option<f32>,retry_wait:f32,is_random:bool,verbose:bool)->Result<(),reqwest::Error>
    {
        let mut cur_depth = 1;
        if verbose {
            println!("-----Starting Directory Recursion-----");
        }

        while cur_depth < depth {
            let mut new_dirs = Vec::new();
            for dirs in dirs_of_dirs{
                if verbose {
                    println!("-----Checking next set of Directories-----");
                }

                for dir in dirs {
                    if verbose {
                        println!("\n-----Current Parsing Directory-----\n{:?}",dir);
                    }
                    //Connect to Directory link
                    url = dir.link.as_str();
                    res = http::Http::connect(client,url,tries,wait,retry_wait,is_random,verbose).await?;
                    //Retrieve Files from Directory Link
                    let files = Scraper::scrape_files(res.as_str(),url,&accept,&reject,verbose);
                    if !files.is_empty(){
                        self.pages.push(asset::Page::new(files));
                    }

                    //Retrieve Directories from current Directory Link
                    let cur_dirs = Scraper::scrape_dirs(res.as_str(),url,verbose);
                    if !cur_dirs.is_empty(){
                        new_dirs.push(cur_dirs);
                    }
                }
                if verbose{
                    println!("-----Finished Checking this set of Directories-----");
                }

            }

            // Check if any Directories were found inside any of the previous Directory Links
            // If there aren't any new Directories, stop checking directories
            if !new_dirs.is_empty(){
                if verbose {
                    println!("-----Setting up new Directories to check-----");
                }
                dirs_of_dirs = new_dirs;
                cur_depth += 1;
            }else{
                if verbose{
                    println!("-----Finished Directory Recursion-----");
                }
                break;
            }

        }
        Ok(())
    }
    /// Scrape the URL that points to a single File.
    fn single_scrape(&mut self,url:&str,verbose:bool) -> bool{
        if util::is_uri(url){
            if verbose{
                println!("URI: {}",url);
            }

            let pages= vec![
                asset::Page::new(
                    vec![asset::File::new(util::remove_last_slash(url).as_str())]
                )
            ];
            self.pages = pages;
            true
        }else{
            false
        }
    }
    /// Read links from a file & start downloading
    pub fn links_from_file(path:&str) -> Vec<PathBuf> {
        let f = fs::read_to_string(path);
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

