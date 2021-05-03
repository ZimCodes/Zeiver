use std::sync::Arc;
use std::rc::Rc;
use std::path::PathBuf;
use std::str::FromStr;
use tokio::time::Duration;
use crossbeam::thread;
mod crawler;
pub mod cmd_opts;

pub struct Zeiver;
impl Zeiver{
    /// Activates Zeiver
    pub fn crawl(){
        let web_crawler = Arc::new(crawler::WebCrawler::new());
        let mut opts = cmd_opts::Opts::new();
        Zeiver::clean_urls_list(& mut opts);
        if !opts.urls.is_empty(){
            Zeiver::multi_thread(web_crawler,opts.urls,opts.record_only,opts.record,opts.test);
        }else{
            let urls = crawler::WebCrawler::get_links(opts.input_file);
            Zeiver::multi_thread(web_crawler,urls,opts.record_only,opts.record,opts.test);
        }
    }
    /// Performs tasks given to the WebCrawler under multi-threading
    /// NOTE: The amount of threads depends on the amount of URLs specified
    /// by the user.
    fn multi_thread(web_crawler:Arc<crawler::WebCrawler>, urls:Vec<PathBuf>,record_only:bool,record:bool,debug:bool){
        thread::scope(|s|{
            let client_builder = reqwest::Client::builder();
            let client = Arc::new(Zeiver::client_creator(client_builder).unwrap());

            for url in urls {
                let web_clone= web_crawler.clone();
                let client_clone = client.clone();
                s.spawn(move |_| {
                    let scraper = web_clone.scraper_task(&client_clone,Some(url));
                    let rc_scraper = Rc::new(scraper);
                    if !debug{
                        if record_only{
                            web_clone.recorder_task(rc_scraper);
                        }else{
                            let rc_scraper_clone= Rc::clone(&rc_scraper);
                            if record{
                                web_clone.recorder_task(rc_scraper_clone);
                            }
                            web_clone.downloader_task(&client_clone,rc_scraper);
                        }
                    }
                });
            }

        }).unwrap();
    }

    // Adds configurations to the Client
    fn client_creator(builder:reqwest::ClientBuilder) -> Result<reqwest::Client,reqwest::Error>{
        let opts = cmd_opts::Opts::new();
        let mut builder = builder;
        // Proxy
        if opts.proxy.is_some(){
            let mut proxy = reqwest::Proxy::all(&opts.proxy.unwrap()).expect("Not a valid proxy!");
            if opts.proxy_auth.is_some(){
                let auth = opts.proxy_auth.unwrap();
                let mut auth_iter = auth.split("$");
                let user = auth_iter.next().expect("Username does not exist");
                let user = user.trim();
                let pass = auth_iter.next().expect("Password does not exist");
                let pass = pass.trim();
                proxy = proxy.basic_auth(user,pass);
            }
            builder = builder.proxy(proxy);
        }
        // Headers
        if opts.headers.is_some(){
            let mut header_map = reqwest::header::HeaderMap::new();
            let headers = opts.headers.unwrap();
            for header in headers{
                let mut head_arr = header.split("$");

                //Set Header Name
                let header_name_str = head_arr.next().expect("Header Name not found!").trim();
                let header_name_lower = header_name_str.to_lowercase();
                let header_name = reqwest::header::HeaderName::from_str(header_name_lower.as_str());
                let header_name = match header_name {
                    Ok(name) => name,
                    Err(e) => {eprintln!("{}",e.to_string());continue;}
                };

                //Set Header Value
                let header_value_str = head_arr.next().expect("Header Value not found!").trim();
                let header_value_lower = header_value_str.to_lowercase();
                let header_value = reqwest::header::HeaderValue::from_str(header_value_lower.as_str());
                let header_value = match header_value{
                    Ok(value) => value,
                    Err(e) => {eprintln!("{}",e.to_string());continue;}
                };
                println!("Name: {} Value: {}",header_name,header_value.to_str().unwrap());
                header_map.insert(header_name,header_value);

            }
            builder = builder.default_headers(header_map);
        }
        // Timeout
        if opts.timeout.is_some(){
            let secs = opts.timeout.unwrap();
            let duration = Duration::from_secs(secs);
            builder = builder.timeout(duration);
        }
        // User Agent
        let user_agent = match opts.user_agent {
            Some(agent) => agent,
            None => format!("Zeiver/{}",env!("CARGO_PKG_VERSION"))
        };
        builder = builder.user_agent(user_agent);
        let policy = reqwest::redirect::Policy::limited(opts.redirects);
        builder.redirect(policy).build()
    }
    // Removes 'zeiver' from the collection of URLs
    fn clean_urls_list(opts: & mut cmd_opts::Opts){
        if let true = opts.urls.contains(&PathBuf::from("zeiver")){
            opts.urls.remove(0);
        }
    }
}


