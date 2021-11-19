use cmd_opts;
use crawler;
use crawler::WebCrawler;
use logger;
use reqwest;
use std::path::PathBuf;
use std::process::Command;
use std::rc::Rc;
use std::str::FromStr;
use tokio::time::Duration;

pub struct Zeiver;

impl Zeiver {
    /// Activates Zeiver
    pub async fn start(opts: cmd_opts::Opts) {
        if opts.update {
            logger::arrows_head("Updating Zeiver! Please wait...");
            Command::new("cargo")
                .args([
                    "install",
                    "--branch",
                    "main",
                    "--git",
                    "https://github.com/ZimCodes/Zeiver",
                ])
                .output()
                .expect("Failed to execute update command for Zeiver");
            logger::arrows_head("Update Completed!");
        } else {
            Zeiver::crawl(opts).await;
        }
    }
    ///Begin crawling process
    async fn crawl(mut opts: cmd_opts::Opts) {
        let web_crawler = Rc::new(crawler::WebCrawler::new(opts.clone()));
        Zeiver::clean_urls_list(&mut opts);
        if !opts.urls.is_empty() {
            Zeiver::multi_thread(web_crawler, opts).await;
        } else if opts.input_file.is_some() {
            opts.urls = crawler::WebCrawler::input_file_links(&opts.input_file).await;
            Zeiver::multi_thread(web_crawler, opts).await;
        } else {
            web_crawler.recorder_file_task().await;
        }
    }
    /// Performs tasks given to the WebCrawler under multi-threading
    /// NOTE: The amount of threads depends on the amount of URLs specified
    /// by the user.
    async fn multi_thread(web_crawler: Rc<crawler::WebCrawler>, opts: cmd_opts::Opts) {
        let client = Rc::new(Zeiver::client_creator(opts.clone()).unwrap());
        let mut recorder_id: usize = 0;
        for url in opts.urls {
            recorder_id += 1;
            Zeiver::establish_task(
                url,
                web_crawler.clone(),
                client.clone(),
                recorder_id,
                &opts.print_header,
                opts.print_headers,
                opts.print_pages,
                opts.record_only,
                opts.record,
                opts.test,
            )
            .await;
        }
    }
    /// Execute the task provided by the commandline
    async fn establish_task(
        url: PathBuf,
        web_clone: Rc<WebCrawler>,
        client_clone: Rc<reqwest::Client>,
        recorder_id: usize,
        print_header: &Option<String>,
        print_headers: bool,
        print_pages: bool,
        record_only: bool,
        record: bool,
        debug: bool,
    ) {
        if print_headers {
            web_clone
                .print_all_headers(&client_clone, url)
                .await
                .unwrap();
        } else if print_header.is_some() {
            web_clone.print_header(&client_clone, url).await.unwrap();
        } else if print_pages {
            web_clone.print_pages(&client_clone, url).await;
        } else {
            Zeiver::spawn_thread(
                url,
                web_clone,
                client_clone,
                record_only,
                record,
                recorder_id,
                debug,
            )
            .await;
        }
    }
    /// Spawns a new thread
    async fn spawn_thread(
        url: PathBuf,
        web_clone: Rc<WebCrawler>,
        client_clone: Rc<reqwest::Client>,
        record_only: bool,
        record: bool,
        recorder_id: usize,
        debug: bool,
    ) {
        let scraper = web_clone.scraper_task(&client_clone, Some(&url)).await;
        if scraper.is_single_scrape {
            return;
        }
        let rc_scraper = Rc::new(scraper);
        if !debug {
            if record_only {
                web_clone.recorder_task(rc_scraper, recorder_id).await;
            } else {
                let rc_scraper_clone = Rc::clone(&rc_scraper);
                if record {
                    web_clone.recorder_task(rc_scraper_clone, recorder_id).await;
                }
                web_clone.downloader_task(&client_clone, rc_scraper).await;
            }
        }
    }
    // Adds configurations to the Client
    fn client_creator(opts: cmd_opts::Opts) -> Result<reqwest::Client, reqwest::Error> {
        let mut client_builder = reqwest::Client::builder();
        // Proxy
        if opts.proxy.is_some() {
            let mut proxy = reqwest::Proxy::all(&opts.proxy.unwrap()).expect("Not a valid proxy!");
            if opts.proxy_auth.is_some() {
                let auth = opts.proxy_auth.unwrap();
                let mut auth_iter = auth.split("$");
                let user = auth_iter.next().expect("Username does not exist");
                let user = user.trim();
                let pass = auth_iter.next().expect("Password does not exist");
                let pass = pass.trim();
                proxy = proxy.basic_auth(user, pass);
            }
            client_builder = client_builder.proxy(proxy);
        }
        // Headers
        if opts.headers.is_some() {
            let mut header_map = reqwest::header::HeaderMap::new();
            let headers = opts.headers.unwrap();
            for header in headers {
                let mut head_arr = header.split("$");

                //Set Header Name
                let header_name_str = head_arr.next().expect("Header Name not found!").trim();
                let header_name_lower = header_name_str.to_lowercase();
                let header_name = reqwest::header::HeaderName::from_str(header_name_lower.as_str());
                let header_name = match header_name {
                    Ok(name) => name,
                    Err(e) => {
                        eprintln!("{}", e.to_string());
                        continue;
                    }
                };

                //Set Header Value
                let header_value_str = head_arr.next().expect("Header Value not found!").trim();
                let header_value_lower = header_value_str.to_lowercase();
                let header_value =
                    reqwest::header::HeaderValue::from_str(header_value_lower.as_str());
                let header_value = match header_value {
                    Ok(value) => value,
                    Err(e) => {
                        eprintln!("{}", e.to_string());
                        continue;
                    }
                };
                logger::log_split("Name", header_name.as_str());
                logger::log_split("Value", header_value.to_str().unwrap());
                logger::new_line();
                header_map.insert(header_name, header_value);
            }
            client_builder = client_builder.default_headers(header_map);
        }
        // Timeout
        if opts.timeout.is_some() {
            let secs = opts.timeout.unwrap();
            let duration = Duration::from_secs(secs);
            client_builder = client_builder.timeout(duration);
        }
        // User Agent
        let user_agent = match opts.user_agent {
            Some(agent) => agent,
            None => format!("Zeiver/{}", env!("CARGO_PKG_VERSION")),
        };
        client_builder = client_builder.user_agent(user_agent);
        // Accept all certificates
        client_builder = client_builder.danger_accept_invalid_certs(opts.all_certs);
        // Disable Referer
        client_builder = client_builder.referer(false);
        // HTTPS only
        client_builder = client_builder.https_only(opts.https_only);
        // Redirect Policy
        let policy = reqwest::redirect::Policy::limited(opts.redirects);
        client_builder.redirect(policy).build()
    }
    // Removes 'zeiver' from the collection of URLs
    fn clean_urls_list(opts: &mut cmd_opts::Opts) {
        if let true = opts.urls.contains(&PathBuf::from("zeiver")) {
            opts.urls.remove(0);
        }
    }
}
