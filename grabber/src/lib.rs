use logger;
use rand;
use rand::Rng;
use reqwest::Response;
use std::thread;
use std::time::Duration;

pub struct Http;

impl Http {
    /// Establish a connection to the URL
    pub async fn connect(
        client: &reqwest::Client,
        url: &str,
        tries: u32,
        wait: Option<f32>,
        retry_wait: f32,
        is_random: bool,
        verbose: bool,
    ) -> Result<(String, reqwest::StatusCode), reqwest::Error> {
        let res =
            Http::get_response(client, url, tries, wait, retry_wait, is_random, verbose).await?;
        let status = res.status();
        let txt = res.text().await;
        Ok((
            txt.expect("Error while transforming response int text"),
            status,
        ))
    }
    /// Sends a request to the server & receives a response
    pub async fn get_response(
        client: &reqwest::Client,
        url: &str,
        tries: u32,
        wait: Option<f32>,
        retry_wait: f32,
        is_random: bool,
        verbose: bool,
    ) -> Result<Response, reqwest::Error> {
        // Wait between HTTP requests
        if let Some(sec) = wait {
            let mut wait_sec = sec;
            if is_random {
                let mut rng = rand::thread_rng();
                wait_sec = rng.gen_range((0.5 * wait_sec)..=(1.5 * wait_sec));
            }
            Http::pause_thread(wait_sec, verbose);
        }

        let mut error: Option<reqwest::Error> = None;

        // Start sending requests
        for _ in 0..tries {
            match client.get(url).send().await {
                Ok(response) => return Ok(response),
                Err(e) => {
                    if e.is_request() {
                        panic!("Error found with request: {}", e);
                    } else if e.is_builder() {
                        panic!("Invalid domain: [{:?}]. Please check URLs in your input file to make sure they are entered line by line or \
                        check URLs entered in terminal.", e.url());
                    } else {
                        eprintln!("{}. Retrying connection!", e.to_string());
                    }
                    error = Some(e);
                }
            }

            // Wait before sending another request after failing
            Http::pause_thread(retry_wait, verbose);
        }
        Err(error.unwrap())
    }
    /// Prints a header from a Response to the terminal
    pub async fn print_header(
        header: &str,
        client: &reqwest::Client,
        url: &str,
        tries: u32,
        wait: Option<f32>,
        retry_wait: f32,
        is_random: bool,
        verbose: bool,
    ) -> Result<(), reqwest::Error> {
        let response =
            Http::get_response(client, url, tries, wait, retry_wait, is_random, verbose).await?;
        match response.headers().get(header) {
            Some(value) => logger::log_split(header, value.to_str().unwrap()),
            None => logger::log(&format!("{} Header is not available:", header)),
        };
        Ok(())
    }
    /// Prints all Response Headers to the terminal
    pub async fn print_headers(
        client: &reqwest::Client,
        url: &str,
        tries: u32,
        wait: Option<f32>,
        retry_wait: f32,
        is_random: bool,
        verbose: bool,
    ) -> Result<(), reqwest::Error> {
        let response =
            Http::get_response(client, url, tries, wait, retry_wait, is_random, verbose).await?;
        let headers = response.headers();
        logger::log_underline("List of Headers");
        logger::divider();
        for (key, val) in headers.iter() {
            logger::log_split(&format!("{:?}", key), &format!("{:?}", val));
        }
        logger::divider();
        logger::new_line();
        Ok(())
    }
    /// Pauses the thread
    fn pause_thread(wait: f32, verbose: bool) {
        if verbose {
            logger::log("Sleeping!");
        }

        let wait_dur = Duration::from_secs_f32(wait);
        thread::sleep(wait_dur);

        if verbose {
            logger::log("Awake!");
        }
    }
}
