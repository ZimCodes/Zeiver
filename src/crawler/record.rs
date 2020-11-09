use std::fs;
use std::env;
use super::scrape::Scraper;
use std::rc::Rc;
use std::io::Write;


pub struct Recorder{
    scraper:Rc<Scraper>,
    verbose:bool
}

impl Recorder{
    /// Creates a new Recorder
    pub fn new(save:&str,scraper:Rc<Scraper>,verbose:bool) -> Recorder{
        Recorder::save_dir(save);
        Recorder{
            scraper,
            verbose
        }
    }
    /// Create a file and place the corresponding links from each page.
    pub fn run(&self){
        println!("-----Recording Links From Scraper-----");
        let record_file = "URL_Records.txt";
        let mut f = fs::File::create(record_file).expect("Recorder file cannot be created!");
        for page in &self.scraper.pages{
            for file in &page.files{
                let line_separator = if cfg!(target_os = "windows"){
                    "\r\n"
                }else{
                    "\n"
                };
                let link = format!("{}{}",file.link,line_separator);

                if self.verbose{
                    println!("URI: {}",link);
                }

                // Write the link to the page file
                let link_buf = link.as_bytes();
                f.write(link_buf).expect(&*format!("Cannot write to file! File name: {}",file.name));
            }

        }
        println!("-----End of Recording-----");
    }
    /// Set the directory to save downloaded files
    pub fn save_dir(path:&str){
        let is_save_set = env::current_dir().unwrap().as_path().ends_with(path);

        if !is_save_set{
            if let Err(e) = env::set_current_dir(path){
                eprintln!("{}",e.to_string());
                fs::create_dir_all(path).expect(&*format!("directory for path, '{}', cannot be created!",path));
                env::set_current_dir(path).unwrap_or_else(|_e|{
                    env::set_current_dir(".").expect("Cannot set path as a save location!");
                });
            };
        }

        let x = env::current_dir().unwrap();
        println!("Save Directory: {}",x.display());
    }
}