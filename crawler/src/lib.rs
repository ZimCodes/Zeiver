use asset;
use cmd_opts;
use downloader;
use grabber;
use logger;
use recorder;
use reqwest;
use scraper;
use std::path::PathBuf;
use std::rc::Rc;

pub struct WebCrawler {
    opts: cmd_opts::Opts,
}

impl WebCrawler {
    ///Creates a new webcrawler
    pub fn new(opts: cmd_opts::Opts) -> WebCrawler {
        WebCrawler { opts }
    }
    /// Performs task given to the Scraper
    pub async fn scraper_task(
        &self,
        client: &reqwest::Client,
        path: Option<&PathBuf>,
    ) -> scraper::Scraper {
        logger::head("Using Scraper");
        let path = match path {
            Some(pathbuf) => pathbuf,
            None => panic!("No path was specified!"),
        };

        let path = path
            .to_str()
            .expect("Cannot parse PathBuf into a &str in scraper_task.");

        let depth = if self.opts.depth == 0 {
            1usize
        } else {
            self.opts.depth
        };

        self.run_scraper(client, path, depth).await
    }
    /// Activates the Scraper
    async fn run_scraper(
        &self,
        client: &reqwest::Client,
        path: &str,
        depth: usize,
    ) -> scraper::Scraper {
        let mut scraper = scraper::Scraper::new();
        match scraper
            .run(
                client,
                path,
                &self.opts.accept,
                &self.opts.reject,
                depth,
                self.opts.tries,
                self.opts.wait,
                self.opts.retry_wait,
                self.opts.random_wait,
                self.opts.verbose,
            )
            .await
        {
            Ok(is_single_file) => match is_single_file {
                true => {
                    logger::head("Scraper Task Completed!");
                    self.downloader_file_task(client, scraper.files.pop().unwrap())
                        .await
                }
                false => logger::head("Scraper Task Completed!"),
            },
            Err(e) => panic!("{}", e.to_string()),
        };

        scraper
    }
    /// Activates Recorder using content obtained from Scraper
    pub async fn recorder_task(&self, scraper: Rc<scraper::Scraper>, recorder_id: usize) {
        logger::head("Using Recorder");
        let save = self
            .opts
            .output
            .to_str()
            .expect("Cannot parse PathBuf into a &str in downloader task.");
        let mut recorder = recorder::Recorder::new(save, scraper, self.opts.verbose).await;
        recorder
            .run(
                &self.opts.output_record,
                recorder_id,
                self.opts.no_stats_list,
                self.opts.no_stats,
            )
            .await;
        logger::head("Recording Task Completed!");
    }
    /// Activates Recorder using content obtained from user's file
    pub async fn recorder_file_task(&self) {
        logger::head("Using Recorder");
        let save = self
            .opts
            .output
            .to_str()
            .expect("Cannot parse PathBuf into a &str in downloader task.");
        recorder::Recorder::run_from_file(
            &self.opts.input_record,
            &self.opts.output_record,
            save,
            self.opts.no_stats_list,
            self.opts.verbose,
        )
        .await;
        logger::head("Recording Task Completed!");
    }
    /// Retrieves urls from an input file
    pub async fn input_file_links(path: &Option<PathBuf>) -> Vec<PathBuf> {
        recorder::Recorder::links_from_file(path).await
    }

    /// Performs task given to the Downloader
    pub async fn downloader_task(&self, client: &reqwest::Client, scraper: Rc<scraper::Scraper>) {
        logger::head("Using Downloader");
        let downloader = self.init_downloader().await;
        downloader.start(client, scraper).await;
        logger::head("Downloader Task Completed!");
    }
    /// Use Downloader to download a single file
    pub async fn downloader_file_task(&self, client: &reqwest::Client, file: asset::file::File) {
        logger::head("Using Downloader");

        let downloader = self.init_downloader().await;
        if let Err(e) = downloader.start_file(client, file).await {
            logger::error(&*format!("Error while downloading: {}", e.to_string()));
        }
        logger::head("Downloader Task Completed!");
    }
    ///Initialize Downloader
    async fn init_downloader(&self) -> downloader::Downloader {
        let save = self
            .opts
            .output
            .to_str()
            .expect("Cannot parse PathBuf into a &str in downloader task.");
        downloader::Downloader::new(
            save,
            self.opts.cut_dirs,
            self.opts.tries,
            self.opts.wait_download,
            self.opts.retry_wait,
            self.opts.no_dirs,
            self.opts.random_download,
            self.opts.verbose,
        )
        .await
    }
    /// Print header from a request
    pub async fn print_header(
        &self,
        client: &reqwest::Client,
        url: PathBuf,
    ) -> Result<(), reqwest::Error> {
        let url = url.to_string_lossy();
        let header = self.opts.print_header.as_ref().unwrap();
        grabber::Http::print_header(
            header,
            client,
            url.as_ref(),
            self.opts.tries,
            self.opts.wait,
            self.opts.retry_wait,
            self.opts.random_wait,
            self.opts.verbose,
        )
        .await
    }
    /// Print all headers from a request
    pub async fn print_all_headers(
        &self,
        client: &reqwest::Client,
        url: PathBuf,
    ) -> Result<(), reqwest::Error> {
        let url = url.to_string_lossy();
        grabber::Http::print_headers(
            client,
            url.as_ref(),
            self.opts.tries,
            self.opts.wait,
            self.opts.retry_wait,
            self.opts.random_wait,
            self.opts.verbose,
        )
        .await
    }
}
