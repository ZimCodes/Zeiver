use std::fs;
use std::env;
use super::scrape::Scraper;
use std::rc::Rc;
use std::io::{Write, ErrorKind,Error};
use std::collections::HashMap;
use std::path::Path;
mod util;

pub struct Recorder{
    scraper:Rc<Scraper>,
    verbose:bool
}

impl Recorder{
    /// Creates a new Recorder
    pub fn new(save_dir:&str,scraper:Rc<Scraper>,verbose:bool) -> Recorder{
        Recorder::save_dir(&save_dir);
        Recorder{
            scraper,
            verbose,
        }
    }
    /// Create a file and place the corresponding links from each page.
    pub fn run(&mut self,record_file:&String,recorder_id:usize,no_stats:bool){
        println!("-----Recording Links From Scraper-----");
        let record_path = Path::new(record_file);
        let file_name = record_path.file_name().expect("Path to create recorder file does not exist");
        let file_name_str = file_name.to_string_lossy();
        let new_file_path = format!("{}_{}",recorder_id,file_name_str);

        let mut f = fs::File::create(new_file_path).expect("Unable to create record file");
        let mut stats_map:HashMap<&str,u32> = HashMap::new();//{filetype,total} holds recorder stats
        for page in &self.scraper.pages{
            for file in &page.files{
                let line_separator = util::get_line_separator();
                if !no_stats{
                    if let Some(ext) = &file.ext{
                        Recorder::update_stats(&mut stats_map, ext);
                    }
                }
                let link = format!("{}{}",file.link,line_separator);
                if self.verbose{
                    println!("URI: {}",link);
                }
                // Write the link to the page file
                let link_buf = link.as_bytes();
                f.write(link_buf).expect("A problem occurred when trying to write to record file");
            }
        }

        if !no_stats{
            println!("{:?}",stats_map);

            let stats_file = format!(r"{}\{}_stats_{}",env::current_dir().unwrap().to_string_lossy(),recorder_id,file_name_str);
            if let Err(e) = Recorder::create_stats_txt_file(stats_map,&stats_file){
                eprintln!("Cannot make stat file. {}",e);
            }
        }

        println!("-----End of Recording-----");
    }
    /// Set the directory to save downloaded files
    pub fn save_dir(path:&str){
        let is_save_set:bool;
        if path.starts_with("./"){
            is_save_set = env::current_dir().unwrap().as_path().ends_with(&path[2..]);
        }else{
            is_save_set = env::current_dir().unwrap().as_path().ends_with(path);
        }

        if !is_save_set{
            if let Err(e) = env::set_current_dir(path){
                if e.kind() == ErrorKind::NotFound {
                    println!("Creating Directory: \"{}\"",path);
                }else{
                    println!("{}",e);
                }
                fs::create_dir_all(path).expect(&*format!("directory for path, '{}', cannot be created!",path));
                env::set_current_dir(path).unwrap_or_else(|_e|{
                    env::set_current_dir(".").expect("Cannot set path as a save location!");
                });
            };
        }

        let x = env::current_dir().unwrap();
        println!("Save Directory: {}",x.display());
    }
    /// Record the amount of each file type
    fn update_stats<'a>(stats_map:&mut HashMap<&'a str,u32>, ext:&'a str){
        if stats_map.contains_key(&ext) {
            let cur_total = stats_map.get(&ext).unwrap();
            let new_total = cur_total + 1;
            stats_map.insert(ext, new_total);
        }else{
            stats_map.insert(ext, 1);
        }
    }
    // /Create a text file with stats about each URL recorded
    fn create_stats_txt_file(stats_map:HashMap<&str,u32>,record_path:&str)->Result<(),Error>{
        let mut f = fs::File::create(record_path)?;
        let header = "|File Type| -> |Total|";
        let header_line = "----------------------------";
        f.write(format!("{}{}{}{}",
                        header,
                        util::get_line_separator(),
                        header_line,
                        util::get_line_separator()).as_bytes())?;
        let separator = "=================";
        for (file_type,total) in stats_map{
            let message = format!("{} -> {}{}{}{}",
                                  file_type.to_uppercase(),
                                  total,
                                  util::get_line_separator(),
                                  separator,
                                  util::get_line_separator());
            f.write(message.as_bytes())?;
        }
        Ok(())
    }
}