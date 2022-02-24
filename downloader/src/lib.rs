use asset;
use grabber;
use logger;
use mime::Mime;
use recorder::Recorder;
use scraper::Scraper;
use std::rc::Rc;
use std::str::FromStr;
mod util;
pub struct Downloader {
    use_dir: bool,
    cuts: u32,
    tries: u32,
    wait: Option<f32>,
    retry_wait: f32,
    is_random: bool,
    verbose: bool,
}
impl Downloader {
    pub async fn new(
        save: &str,
        cuts: u32,
        tries: u32,
        wait: Option<f32>,
        retry_wait: f32,
        use_dir: bool,
        is_random: bool,
        verbose: bool,
    ) -> Downloader {
        Recorder::save_dir(save).await;
        Downloader {
            use_dir,
            cuts,
            tries,
            wait,
            retry_wait,
            is_random,
            verbose,
        }
    }
    /// Start downloading files from the scraper
    pub async fn start(&self, client: &reqwest::Client, scraper: Rc<Scraper>) {
        if !scraper.files.is_empty() {
            for file in &scraper.files {
                if let Err(e) = self.run(client, file).await {
                    panic!("{}", e.to_string());
                }
            }
        }
    }
    /// Download a file
    pub async fn start_file(
        &self,
        client: &reqwest::Client,
        file: asset::file::File,
    ) -> Result<(), reqwest::Error> {
        self.run(client, &file).await
    }
    /// Downloads a File
    async fn run(
        &self,
        client: &reqwest::Client,
        file: &asset::file::File,
    ) -> Result<(), reqwest::Error> {
        let res = grabber::Http::response(
            client,
            &file.link,
            self.tries,
            self.wait,
            self.retry_wait,
            self.is_random,
            self.verbose,
        )
        .await?;
        let headers = res.headers();
        let content_type = headers.get(reqwest::header::CONTENT_TYPE);
        match content_type {
            None => {
                logger::error("The response does not contain a Content-Type header.");
            }
            Some(content) => {
                let content_type = Mime::from_str(
                    content
                        .to_str()
                        .expect("cannot parse header value into &str"),
                )
                .expect("Cannot parse header value into a Mime type");
                let res_content = match content_type.type_() {
                    mime::TEXT => Box::new(util::HttpBodyType::Text(res.text().await?)),
                    _ => Box::new(util::HttpBodyType::Binary(res.bytes().await?)),
                };
                logger::head(&format!("Downloading {}", file.name));
                util::prepare_file(res_content, file, self.cuts, self.use_dir).await;
            }
        };

        Ok(())
    }
}
