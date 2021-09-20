use reqwest;
use asset;
use http;

mod parser;
mod od;
mod search;

pub struct Scraper {
    pub pages: Vec<asset::page::Page>,
    dir_links: Vec<String>,
    od_type: od::ODMethod,
    current_subpage: usize,
}

impl Scraper {
    pub fn new() -> Scraper {
        let pages = Vec::new();
        let dir_links = Vec::new();
        let od_type = od::ODMethod::Generic;
        let current_subpage = 1;
        Scraper {
            pages,
            dir_links,
            od_type,
            current_subpage,
        }
    }
    /// Scrape files URLs present on the current page(URL)
    fn scrape_files(&mut self, res: &str, url: &str, accept: &Option<String>, reject: &Option<String>, is_first_parse: bool, verbose: bool)
                    -> Vec<asset::file::File>
    {
        let mut files: Vec<asset::file::File> = Vec::new();
        let mut previous_file = String::new();//variable to check for duplicates
        let mut previous_mod_file = String::new();//variable after modification to check for duplicates
        if is_first_parse {
            println!("-----First File Parse-----");
        } else {
            println!("-----Parsing File Links-----");
        }
        let scraped_links = search::filtered_links(res, url, &self.od_type);
        for x in scraped_links {
            if previous_file != x {
                previous_file = x.to_string();
                let is_file_ext = parser::is_file_ext(x.as_str());
                let ending_check = is_file_ext
                    || od::olaindex::OLAINDEX::has_dl_query(&x)
                    || od::olaindex::OLAINDEX::hash_query(&x);

                let sub_check = parser::sub_dir_check(&x, url);

                if ending_check
                    && !x.ends_with("/")
                    && (!x.starts_with("http")
                    || sub_check
                    || self.od_type.eq(&od::ODMethod::Apache)
                    || self.od_type.eq(&od::ODMethod::NGINX))
                {
                    if !x.starts_with("?dir=")
                        || (x.starts_with("?dir=") && parser::check_dir_query(url, x.as_str()))
                    {
                        let mut x = String::from(x);
                        x = self.od_file_link_modify(url, &x, res);
                        if previous_mod_file != x {
                            if od::olaindex::OLAINDEX::has_dl_query(&x) {
                                let (new_accept, new_reject) = od::olaindex::OLAINDEX::acc_rej_filters(&accept, &reject);
                                Scraper::acc_rej_check(url, &mut files, &x, &new_accept, &new_reject, verbose);
                            } else {
                                Scraper::acc_rej_check(url, &mut files, &x, accept, reject, verbose);
                            }
                            //Assign as the new previously modded file
                            previous_mod_file = x;
                        }
                    }
                }
            }
        }
        println!("--->| # of Files: {} |<---\n", files.len());
        println!("-----End of Parsing File Links-----");
        files
    }
    /// Scrape directory URLs present on the current page(URL)
    fn scrape_dirs(&mut self, res: &str, url: &str, is_first_parse: bool, verbose: bool) -> Vec<asset::directory::Directory> {
        let mut dirs = Vec::new();
        let mut past_dir = String::new();//variable to check for duplicates
        if is_first_parse {
            println!("-----First Directory Parse-----");
        } else {
            println!("-----Parsing Directory Links-----");
        }
        let scraped_links = search::filtered_links(res, url, &self.od_type);
        for x in scraped_links {
            let x = &*x;
            let current_dir = format!("{}{}", url, x);
            if past_dir != current_dir {
                past_dir = current_dir;

                if (!x.starts_with("http")
                    || x.starts_with(url))
                    && !parser::is_back_url(x)
                    && !parser::is_home_url(x)
                    && !parser::unrelated_dir_queries(x)
                    && !parser::is_rel_url(url, x)
                    && !parser::has_page_query(x)
                    || parser::within_page_limit(x, self.current_subpage)
                {
                    if parser::is_not_symbol(x)
                        && !od::olaindex::OLAINDEX::has_dl_query(&x)
                        && !parser::is_file_ext(x)
                    {
                        self.add_dir(url, x, &mut dirs, verbose);
                    } else if x.starts_with("?dir=") && parser::check_dir_query(url, x) {
                        self.add_dir(url, x, &mut dirs, verbose);
                    }
                }
            }
        }
        println!("--->| # of Directories: {} |<---\n", dirs.len());
        println!("-----End of Parsing Directory Links-----");
        dirs
    }
    pub async fn run(&mut self, client: &reqwest::Client, url: &str, accept: &Option<String>,
                     reject: &Option<String>, depth: usize, tries: u32, wait: Option<f32>,
                     retry_wait: f32, is_random: bool, verbose: bool)
                     -> Result<(), reqwest::Error>
    {
        let url_string = parser::add_last_slash(url);
        let url = url_string.as_str();
        //Check if URL points to a file
        if self.single_scrape(url, verbose) {
            return Ok(());
        }

        self.od_type_from_url(url);

        //Determine od type from url
        let url = parser::sanitize_url(url);
        //Retrieve page
        let res = http::Http::connect(client, &url, tries, wait, retry_wait, is_random, verbose).await?;

        //Determine od type from html document
        self.od_type_from_document(&*res,client, &url, tries, wait, retry_wait, is_random, verbose).await?;

        let dirs_of_dirs = vec![self.scrape_dirs(res.as_str(), &url, true, verbose)];

        let files = self.scrape_files(res.as_str(), &url, &accept, &reject, true, verbose);
        if !files.is_empty() {
            self.pages.push(asset::page::Page::new(files));
        }

        //Determines whether to start recursive scraping
        let is_dir_empty = dirs_of_dirs.get(0).unwrap().is_empty();
        if !is_dir_empty {
            self.dir_recursive(client, &url, res, dirs_of_dirs, accept, reject, depth, tries, wait, retry_wait, is_random, verbose).await?;
        }

        Ok(())
    }
    /// Adds a URL to the list of directories cycle through
    fn add_dir(&mut self, url: &str, x: &str, dirs: &mut Vec<asset::directory::Directory>, verbose: bool) {
        let joined_url = if x.starts_with("http") {
            String::from(x)
        } else {
            parser::url_joiner(url, x)
        };

        if !self.dir_links.contains(&joined_url) {
            if verbose {
                println!("DIR: {}", joined_url);
            }
            self.dir_links.push(joined_url.clone());
            dirs.push(
                asset::directory::Directory::new(
                    format!("{}", joined_url)
                )
            );
        }
    }
    /// Adds a URL to the list of files to download
    fn add_file(url: &str, x: &str, files: &mut Vec<asset::file::File>, verbose: bool) {
        let joined_url = if x.starts_with("http") {
            String::from(x)
        } else {
            parser::url_joiner(url, x)
        };
        if verbose {
            println!("URI: {}", joined_url);
        }
        files.push(
            asset::file::File::new(
                joined_url.as_str()
            ));
    }
    fn od_type_from_url(&mut self, url: &str) {
        println!("-----Resolving Scrape Method-----");
        self.od_type = od::od_type_from_url(url);
    }
    async fn od_type_from_document(&mut self, res: &str,client: &reqwest::Client, url: &str, tries: u32, wait: Option<f32>,
                             retry_wait: f32, is_random: bool, verbose: bool) -> Result<(), reqwest::Error> {
        if self.od_type.eq(&od::ODMethod::None) {
            let response = http::Http::get_response(client, &url, tries, wait, retry_wait, is_random, verbose).await?;
            let server_name = match response.headers().get("server") {
                Some(header_value) => header_value.to_str().unwrap(),
                None => ""
            };
            self.od_type = od::od_type_from_document(res, server_name);
        }
        println!("----->  {:?}  <-----\n", self.od_type);
        Ok(())
    }
    fn od_file_link_modify(&self, url: &str, x: &String, res: &str) -> String {
        if self.od_type.eq(&od::ODMethod::OLAINDEX) {
            od::olaindex::OLAINDEX::transform_link(x)
        } else if self.od_type.eq(&od::ODMethod::AutoIndexPHP) {
            od::autoindex_php::AutoIndexPHP::transform_dl_link(url, x.as_str(), res)
        } else {
            x.to_string()
        }
    }
    /// Recursively scrape file URLs from child directories
    /// NOTE: variables ARE being used
    #[allow(unused_assignments)]
    async fn dir_recursive(&mut self, client: &reqwest::Client, mut url: &str, mut res: String, mut dirs_of_dirs: Vec<Vec<asset::directory::Directory>>,
                           accept: &Option<String>, reject: &Option<String>, depth: usize, tries: u32, wait: Option<f32>, retry_wait: f32, is_random: bool, verbose: bool) -> Result<(), reqwest::Error>
    {
        println!("-----Starting Directory Diving-----");

        let mut cur_depth = 1;
        while cur_depth < depth {
            let mut new_dirs = Vec::new();
            for dirs in dirs_of_dirs {
                println!("-----Checking next set of Directories-----");

                for dir in dirs {
                    println!("\n-----Current Parsing Directory-----\n{:?}", dir);

                    //Connect to Directory link
                    url = dir.link.as_str();

                    res = http::Http::connect(client, url, tries, wait, retry_wait, is_random, verbose).await?;
                    //Retrieve Files from Directory Link
                    let files = self.scrape_files(res.as_str(), url, &accept, &reject, false, verbose);
                    if !files.is_empty() {
                        self.pages.push(asset::page::Page::new(files));
                    }

                    //Retrieve Directories from current Directory Link
                    let cur_dirs = self.scrape_dirs(res.as_str(), url, false, verbose);
                    if !cur_dirs.is_empty() {
                        new_dirs.push(cur_dirs);
                    }
                }

                println!("-----Finished Checking this set of Directories-----");
            }

            // Check if any Directories were found inside any of the previous Directory Links
            // If there aren't any new Directories, stop checking directories
            if !new_dirs.is_empty() {
                println!("-----Setting up new Directories to check-----");

                dirs_of_dirs = new_dirs;
                cur_depth += 1;
            } else {
                println!("-----Finished Directory Diving-----");
                break;
            }
        }
        Ok(())
    }
    /// Scrape the URL that points to a single File.
    fn single_scrape(&mut self, url: &str, verbose: bool) -> bool {
        if parser::is_uri(url) {
            if verbose {
                println!("URI: {}", url);
            }

            let pages = vec![
                asset::page::Page::new(
                    vec![asset::file::File::new(parser::remove_last_slash(url).as_str())]
                )
            ];
            self.pages = pages;
            true
        } else {
            false
        }
    }
    /// Scrape files based on 'accept' and 'reject' option configurations
    fn acc_rej_check(url: &str, files: &mut Vec<asset::file::File>, x: &String, accept: &Option<String>, reject: &Option<String>, verbose: bool) {
        //Accept Option
        if accept.is_some() {
            let reg = parser::set_regex(accept);
            if reg.is_match(x.as_str()) {
                Scraper::add_file(url, x.as_str(), files, verbose);
            }
        }//Reject Option
        else if reject.is_some() {
            let reg = parser::set_regex(reject);
            if !reg.is_match(x.as_str()) {
                Scraper::add_file(url, x.as_str(), files, verbose);
            }
        }//Neither Option
        else if accept.is_none() && reject.is_none() {
            Scraper::add_file(url, x.as_str(), files, verbose);
        }
    }
}
