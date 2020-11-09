use reqwest::Response;
use std::thread;
use std::time::Duration;
use rand;
use rand::Rng;

pub struct Http;

impl Http{
    /// Establish a connection to the URL
    pub async fn connect(client:&reqwest::Client,url:&str,tries:u32,wait:Option<f32>,retry_wait:f32,is_random:bool)
        -> Result<String,reqwest::Error> {
        let res = Http::get_response(client,url,tries,wait,retry_wait,is_random).await?;
        res.text().await
    }
    /// Sends a request to the server
    pub async fn get_response(client:&reqwest::Client,url:&str,tries:u32,wait:Option<f32>,retry_wait:f32,is_random:bool) -> Result<Response,reqwest::Error>{
        // Wait between HTTP requests

        if let Some(sec) = wait{

            let mut wait_sec = sec;
            if is_random{
                let mut rng = rand::thread_rng();
                wait_sec = rng.gen_range(0.5 * wait_sec, 1.5 * wait_sec);
            }
            Http::pause_thread(wait_sec);

        }


        let mut error:Option<reqwest::Error> = None;

        // Start sending requests
        for _ in 0..tries {
            match client.get(url).send().await {
                Ok(response) => return Ok(response),
                Err(e) => {
                    if e.is_request(){
                        panic!("Error found with request: {}",e);
                    }
                    eprintln!("{}. Retrying connection!", e.to_string());
                    error = Some(e);
                }
            }

            // Wait before sending another request after failing
            Http::pause_thread(retry_wait);
        }
        Err(error.unwrap())
    }
    /// Pauses the thread
    fn pause_thread(wait:f32){
        println!("Sleeping!");
        let wait_dur = Duration::from_secs_f32(wait);
        thread::sleep(wait_dur);
        println!("Awake!");
    }
}