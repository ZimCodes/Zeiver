use std::path::PathBuf;
use crate::cmd_opts;
use std::rc::Rc;
mod scrape;
mod download;
pub mod asset;
pub mod http;
mod record;

pub struct WebCrawler {
    opts:cmd_opts::Opts
}
impl WebCrawler {
    pub fn new() -> WebCrawler {
        WebCrawler {
            opts:cmd_opts::Opts::new()
        }
    }
    /// Performs task given to the Scraper
    pub fn scraper_task(&self,client:&reqwest::Client,path:Option<PathBuf>)-> scrape::Scraper{
        let path = match path{
            Some(pathbuf)=> pathbuf,
            None=> panic!("No path was specified!")
        };
        let path = path.to_str().expect("Cannot parse PathBuf into a &str in scraper_task.");
        let depth = if self.opts.depth == 0 {
            1usize
        }else{
          self.opts.depth
        };
        WebCrawler::run_scraper(client, path, &self.opts.accept, &self.opts.reject, depth,
                                self.opts.tries, self.opts.wait, self.opts.retry_wait, self.opts.random_wait, self.opts.verbose)
    }
    /// Performs task given to the Downloader
    pub fn downloader_task(&self,client:&reqwest::Client,scraper:Rc<scrape::Scraper>){
        let save = self.opts.output.to_str().expect("Cannot parse PathBuf into a &str in downloader task.");
        WebCrawler::run_downloader(&client, scraper, save, self.opts.cut_dirs, self.opts.tries,self.opts.wait,self.opts.retry_wait,self.opts.no_dirs,
                                   self.opts.random_wait,self.opts.verbose);
    }
    pub fn recorder_task(&self,scraper:Rc<scrape::Scraper>,recorder_id:usize){
        let save = self.opts.output.to_str().expect("Cannot parse PathBuf into a &str in downloader task.");
        let mut recorder = record::Recorder::new(save, scraper, self.opts.verbose);
        recorder.run(&self.opts.record_file,recorder_id,self.opts.no_stats);
    }
    //Activates the Scraper
    fn run_scraper(client:&reqwest::Client,path:&str,accept:&Option<String>,reject:&Option<String>,depth:usize,tries:u32,wait:Option<f32>,retry_wait:f32,is_random:bool,verbose:bool)
                   -> scrape::Scraper
    {
        let mut scraper = scrape::Scraper::new();
        if let Err(e) = scraper.run(client,path,accept,reject,depth,tries,wait,retry_wait,is_random,verbose){
            panic!("{}",e.to_string());
        }
        scraper
    }
    /// Activates the Downloader
    fn run_downloader(client:&reqwest::Client,scraper:Rc<scrape::Scraper>,save:&str,cuts:u32,tries:u32,wait:Option<f32>,retry_wait:f32,
                      use_dir:bool,is_random:bool,verbose:bool){
        let downloader = download::Downloader::new(save,cuts,tries,wait,retry_wait,use_dir,is_random,verbose);
        downloader.start(client,scraper);
    }
    /// Retrieves urls from an input file
    pub fn get_links(path:Option<PathBuf>) ->Vec<PathBuf>{
        scrape::Scraper::links_from_file(path.unwrap().to_str().expect("Cannot parse links from file into a string"))
    }
}
