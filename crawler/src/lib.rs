use std::path::PathBuf;
use cmd_opts;
use std::sync::Arc;
use reqwest;
use scraper;
use downloader;
use recorder;
use grabber;
use logger;

pub struct WebCrawler {
    opts:cmd_opts::Opts
}
impl WebCrawler {
    pub fn new(opts: cmd_opts::Opts) -> WebCrawler {
        WebCrawler {
            opts
        }
    }
    /// Performs task given to the Scraper
    pub async fn scraper_task(&self,client:&reqwest::Client,path:Option<&PathBuf>) -> scraper::Scraper{
        logger::head("Using Scraper");
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
                                self.opts.tries, self.opts.wait, self.opts.retry_wait, self.opts.random_wait, self.opts.verbose).await
    }
    /// Performs task given to the Downloader
    pub async fn downloader_task(&self,client:&reqwest::Client,scraper:Arc<scraper::Scraper>){
        logger::head("Using Downloader");
        let save = self.opts.output.to_str().expect("Cannot parse PathBuf into a &str in downloader task.");
        WebCrawler::run_downloader(&client, scraper, save, self.opts.cut_dirs, self.opts.tries,self.opts.wait,self.opts.retry_wait,self.opts.no_dirs,
                                   self.opts.random_wait,self.opts.verbose).await;
        logger::head("Downloader Task Completed!");
    }
    /// Activates Recorder using content obtained from Scraper
    pub async fn recorder_task(&self,scraper:Arc<scraper::Scraper>,recorder_id:usize){
        logger::head("Using Recorder");
        let save = self.opts.output.to_str().expect("Cannot parse PathBuf into a &str in downloader task.");
        let mut recorder = recorder::Recorder::new(save, scraper, self.opts.verbose).await;
        recorder.run(&self.opts.output_record, recorder_id, self.opts.no_stats_list,self.opts.no_stats).await;
        logger::head("Recording Task Completed!");
    }
    /// Activates Recorder using content obtained from user's file
    pub async fn recorder_file_task(&self){
        logger::head("Using Recorder");
        let save = self.opts.output.to_str().expect("Cannot parse PathBuf into a &str in downloader task.");
        recorder::Recorder::run_from_file(&self.opts.input_record,&self.opts.output_record,save,self.opts.no_stats_list,self.opts.verbose).await;
        logger::head("Recording Task Completed!");
    }
    /// Activates the Scraper
    async fn run_scraper(client:&reqwest::Client,path:&str,accept:&Option<String>,reject:&Option<String>,depth:usize,tries:u32,wait:Option<f32>,retry_wait:f32,is_random:bool,verbose:bool)
                   -> scraper::Scraper
    {
        let mut scraper = scraper::Scraper::new();
        if let Err(e) = scraper.run(client,path,accept,reject,depth,tries,wait,retry_wait,is_random,verbose).await{
            panic!("{}",e.to_string());
        }
        logger::head("Scraper Task Completed!");
        scraper
    }
    /// Activates the Downloader
    async fn run_downloader(client:&reqwest::Client,scraper:Arc<scraper::Scraper>,save:&str,cuts:u32,tries:u32,wait:Option<f32>,retry_wait:f32,
                      use_dir:bool,is_random:bool,verbose:bool){
        let downloader = downloader::Downloader::new(save,cuts,tries,wait,retry_wait,use_dir,is_random,verbose).await;
        downloader.start(client,scraper).await;
    }
    /// Retrieves urls from an input file
    pub async fn input_file_links(path: &Option<PathBuf>) ->Vec<PathBuf>{
        recorder::Recorder::links_from_file(path).await
    }
    pub async fn print_header(header:&String,client:&reqwest::Client,url:PathBuf,tries:u32,wait:Option<f32>,retry_wait:f32,is_random:bool,
                              verbose:bool) -> Result<(),reqwest::Error>{
        let url = url.to_string_lossy();
        grabber::Http::print_header(header,client,url.as_ref(),tries,wait,retry_wait,is_random,verbose).await
    }
    pub async fn print_all_headers(client:&reqwest::Client,url:PathBuf,tries:u32,wait:Option<f32>,retry_wait:f32,is_random:bool,
                                   verbose:bool) -> Result<(),reqwest::Error>{
        let url = url.to_string_lossy();
        grabber::Http::print_headers(client,url.as_ref(),tries,wait,retry_wait,is_random,verbose).await
    }
}
