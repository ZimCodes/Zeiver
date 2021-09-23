use tokio::fs;
use std::env;
use scraper::{self,Scraper};
use std::sync::Arc;
use tokio::io::{AsyncWriteExt,Error,ErrorKind};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use asset;

mod util;

pub struct Recorder{
    scraper:Arc<Scraper>,
    verbose:bool
}

impl Recorder{
    /// Creates a new Recorder
    pub async fn new(save_dir:&str,scraper:Arc<Scraper>,verbose:bool) -> Recorder{
        Recorder::save_dir(&save_dir).await;
        Recorder{
            scraper,
            verbose,
        }
    }
    pub async fn run_from_file(input_record:&Option<PathBuf>,output_record:&String,save_dir:&str,verbose:bool){
        Recorder::save_dir(&save_dir).await;
        Recorder::run_file(input_record,output_record,1,verbose).await;
    }
    /// Create a file and place the corresponding links from each page.
    pub async fn run(&mut self, output_record:&String, recorder_id:usize, no_stats_list:bool, no_stats:bool){
        println!("\n-----Recording Links From Scraper-----\n");
        let (file_name_str,new_file_path) = Recorder::file_properties(output_record,recorder_id);

        let mut f = fs::File::create(new_file_path).await.expect("Unable to create record file");
        let mut file_type_map:HashMap<String,u32> = HashMap::new();//{filetype,total} holds recorder stats
        let mut file_vec = Vec::new();
        for page in &self.scraper.pages{
            for file in &page.files{
                let line_separator = util::get_line_separator();
                if !no_stats{
                    if let Some(ext) = &file.ext{
                        let file_name = &file.name;
                        let new_name = String::from(file_name);
                        if !no_stats_list{
                            file_vec.push(new_name);
                        }
                        Recorder::update_stats(&mut file_type_map, String::from(ext));
                    }
                }
                let link = format!("{}{}",file.link,line_separator);
                if self.verbose{
                    println!("URI: {}",link);
                }
                // Write the link to the page file
                let link_buf = link.as_bytes();
                f.write(link_buf).await.expect("A problem occurred when trying to write to record file");
            }
        }

        if !no_stats{
            Recorder::stat_tasks(file_type_map, file_vec, recorder_id, &file_name_str, self.verbose).await;
        }

        println!("-----End of Recording-----");
    }
    /// Create stats based on input file
    async fn run_file(input_record:&Option<PathBuf>,output_record:&String,recorder_id:usize,verbose:bool){
        println!("\n-----Recording Links From File-----\n");
        let (file_name_str,_) = Recorder::file_properties(output_record,recorder_id);
        let mut file_type_map:HashMap<String,u32> = HashMap::new();//{filetype,total} holds recorder stats
        let mut file_vec = Vec::new();
        let paths = Recorder::links_from_file(input_record).await;
        for pathbuf in paths{
            let path = pathbuf.as_path();
            if verbose{
                println!("{}\n",path.display());
            }
            if let Some(ext) = Recorder::pathbuf_to_extension(path){
                let file_path = pathbuf.to_str().unwrap();
                let file = asset::file::File::new(file_path);
                file_vec.push(file.name);
                Recorder::update_stats(&mut file_type_map, ext);
            }
        }
        Recorder::stat_tasks(file_type_map, file_vec, recorder_id, &file_name_str, verbose).await;
        println!("-----End of Recording-----");
    }
    fn pathbuf_to_extension(path: &Path) -> Option<String> {
        if path.extension().is_none(){
            return None;
        }
        let os_path = path.extension().unwrap();
        Some(String::from(os_path.to_string_lossy()))
    }
    /// Read links from a file & start downloading
    pub async fn links_from_file(input_file: &Option<PathBuf>) -> Vec<PathBuf> {
        let path_ref = input_file.as_ref().unwrap();
        let path_str = path_ref.to_str().expect("Cannot parse links from file into a string");
        let f = fs::read_to_string(path_str).await;
        let msg = match f {
            Ok(msg) => msg,
            Err(e) => match e.kind() {
                ErrorKind::NotFound => panic!("File cannot be found!"),
                ErrorKind::InvalidData => panic!("The contents of the file are not valid UTF-8"),
                _ => {
                    panic!("Error retrieving data from file")
                }
            }
        };
        let links = if cfg!(target_os = "windows") {
            msg.split("\r\n")
        } else {
            msg.split("\n")
        };
        let mut link_strings = Vec::new();
        for link in links {
            link_strings.push(PathBuf::from(link))
        }
        link_strings
    }
    /// Setup files
    fn file_properties(output_record:&String,recorder_id:usize)-> (String,String){
        let record_path = Path::new(output_record);
        let file_name = record_path.file_name().expect("Path to create recorder file does not exist");
        let file_name_str = file_name.to_string_lossy();
        let new_file_path = format!("{}_{}",recorder_id,file_name_str);
        (String::from(file_name_str),new_file_path)
    }
    /// Set the directory to save downloaded files
    pub async fn save_dir(path:&str){
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
                    eprintln!("{}",e);
                }
                fs::create_dir_all(path).await.expect(&*format!("directory for path, '{}', cannot be created!",path));
                env::set_current_dir(path).unwrap_or_else(|_e|{
                    env::set_current_dir(".").expect("Cannot set path as a save location!");
                });
            };
        }

        let x = env::current_dir().unwrap();
        println!("Save Directory: {}",x.display());
    }
    /// Start stat operations
    async fn stat_tasks(file_type_map:HashMap<String,u32>, file_vec:Vec<String>, recorder_id:usize, file_name:&str, verbose:bool){
        if verbose{
            println!("{:?}", file_type_map);
        }

        let stats_file = format!(r"{}\{}_stats_{}",env::current_dir().unwrap().to_string_lossy(),recorder_id,file_name);
        if let Err(e) = Recorder::create_stats_file(file_type_map, file_vec, &stats_file).await{
            eprintln!("Cannot make stat file. {}",e);
        }
    }
    /// Record the amount of each file type
    fn update_stats(stats_map:&mut HashMap<String,u32>, ext:String){
        if stats_map.contains_key(&ext) {
            let cur_total = stats_map.get(&ext).unwrap();
            let new_total = cur_total + 1;
            stats_map.insert(ext, new_total);
        }else{
            stats_map.insert(ext, 1);
        }
    }
    // /Create a text file with stats about each URL recorded
    async fn create_stats_file(file_type_map:HashMap<String,u32>, file_vec:Vec<String>, record_path:&str) -> Result<(), Error> {
        let mut f = fs::File::create(record_path).await?;
        let header_line = "----------------------------";
        Recorder::add_file_type_stats(&mut f,file_type_map,header_line).await?;
        if !file_vec.is_empty(){
            Recorder::add_file_name_stats(&mut f, file_vec,header_line).await
        }else{
            Ok(())
        }
    }
    async fn add_file_type_stats(f: &mut fs::File, file_type_map:HashMap<String,u32>,header_line:&str) ->Result<(),Error>{
        let header = "\t|File Type| -> |Total|";

        let separator = "======================";
        f.write(format!("{}{}{}{}",
                        header,
                        util::get_line_separator(),
                        header_line,
                        util::get_line_separator()).as_bytes()).await?;
        for (file_type,total) in file_type_map{
            let message = format!("{} -> {}{}{}{}",
                                  file_type.to_uppercase(),
                                  total,
                                  util::get_line_separator(),
                                  separator,
                                  util::get_line_separator());
            f.write(message.as_bytes()).await?;
        }
        Ok(())
    }
    async fn add_file_name_stats(f:&mut fs::File,file_vec:Vec<String>,header_line:&str) -> Result<(),Error>{
        let mut sort_vec:Vec<&String> = file_vec
            .iter()
            .filter(|file_name| !file_name.is_empty())
            .map(|file_name| file_name).collect();
        sort_vec.sort_unstable();
        let header = format!("{}\t{} File Names in order {}{}{}{}",
                             util::get_line_separator(),
                             "♦️",
                             "♦️",
                             util::get_line_separator(),
                             header_line,
                             util::get_line_separator()
        );
        f.write(header.as_bytes()).await?;
        for pathbuf in sort_vec{
            let message = format!("{}{}",
                                  pathbuf,
                                  util::get_line_separator());
            f.write(message.as_bytes()).await?;
        }
        Ok(())
    }
}