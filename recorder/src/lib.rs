use asset;
use logger;
use scraper::{self, Scraper};
use std::env;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::fs;
use tokio::io::{AsyncWriteExt, Error, ErrorKind};

mod util;

pub struct Recorder {
    scraper: Arc<Scraper>,
    verbose: bool,
}

impl Recorder {
    /// Creates a new Recorder
    pub async fn new(save_dir: &str, scraper: Arc<Scraper>, verbose: bool) -> Recorder {
        Recorder::save_dir(&save_dir).await;
        Recorder { scraper, verbose }
    }
    pub async fn run_from_file(
        input_record: &Option<PathBuf>,
        output_record: &String,
        save_dir: &str,
        no_stats_list: bool,
        verbose: bool,
    ) {
        Recorder::save_dir(&save_dir).await;
        Recorder::run_file(input_record, output_record, 1, no_stats_list, verbose).await;
    }
    /// Create a file and place the corresponding links from each page.
    pub async fn run(
        &mut self,
        output_record: &String,
        recorder_id: usize,
        no_stats_list: bool,
        no_stats: bool,
    ) {
        logger::new_line();
        logger::head("Recording Links From Scraper");
        logger::new_line();
        let (file_name_str, new_file_path) = Recorder::file_properties(output_record, recorder_id);

        let mut f = fs::File::create(new_file_path)
            .await
            .expect("Unable to create record file");
        let mut stat = asset::stat::Stat::new();
        for page in &self.scraper.pages {
            for file in &page.files {
                let line_separator = util::get_line_separator();
                if !no_stats {
                    if let Some(ext) = &file.ext {
                        let file_name = &file.name;
                        let new_name = String::from(file_name);
                        if !no_stats_list {
                            stat.add_file(new_name);
                        }
                        stat.add_extension(ext.to_string());
                    }
                }
                let link = format!("{}{}", file.link, line_separator);
                if self.verbose {
                    logger::log_split("URI", &file.link);
                }
                // Write the link to the page file
                let link_buf = link.as_bytes();
                f.write(link_buf)
                    .await
                    .expect("A problem occurred when trying to write to record file");
            }
        }

        if !no_stats {
            Recorder::stat_tasks(stat, recorder_id, &file_name_str, self.verbose).await;
        }

        logger::head("End of Recording");
    }
    /// Create stats based on input file
    async fn run_file(
        input_record: &Option<PathBuf>,
        output_record: &String,
        recorder_id: usize,
        no_stats_list: bool,
        verbose: bool,
    ) {
        logger::new_line();
        logger::head("Recording Links From File");
        logger::new_line();
        let (file_name_str, _) = Recorder::file_properties(output_record, recorder_id);
        let mut stat = asset::stat::Stat::new();
        let paths = Recorder::links_from_file(input_record).await;
        for pathbuf in paths {
            let path = pathbuf.as_path();
            if verbose {
                logger::log(&format!("{}", path.display()));
                logger::new_line();
            }
            if let Some(ext) = Recorder::pathbuf_to_extension(path) {
                let file_path = pathbuf.to_str().unwrap();
                let file = asset::file::File::new(file_path);
                if !no_stats_list {
                    stat.add_file(file.name);
                }
                stat.add_extension(ext);
            }
        }
        Recorder::stat_tasks(stat, recorder_id, &file_name_str, verbose).await;
    }
    fn pathbuf_to_extension(path: &Path) -> Option<String> {
        if path.extension().is_none() {
            return None;
        }
        let os_path = path.extension().unwrap();
        Some(String::from(os_path.to_string_lossy()))
    }
    /// Read links from a file & start downloading
    pub async fn links_from_file(input_file: &Option<PathBuf>) -> Vec<PathBuf> {
        let path_ref = input_file.as_ref().unwrap();
        let path_str = path_ref
            .to_str()
            .expect("Cannot parse links from file into a string");
        let f = fs::read_to_string(path_str).await;
        let msg = match f {
            Ok(msg) => msg,
            Err(e) => match e.kind() {
                ErrorKind::NotFound => panic!("File cannot be found!"),
                ErrorKind::InvalidData => panic!("The contents of the file are not valid UTF-8"),
                _ => {
                    panic!("Error retrieving data from file")
                }
            },
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
    fn file_properties(output_record: &String, recorder_id: usize) -> (String, String) {
        let record_path = Path::new(output_record);
        let file_name = record_path
            .file_name()
            .expect("Path to create recorder file does not exist");
        let file_name_str = file_name.to_string_lossy();
        let file = asset::file::File::new(&format!("https://example.co/{}", file_name_str));
        let new_file_path = format!("{}_{}", recorder_id, file_name_str);
        (file.to_json(), new_file_path)
    }
    /// Set the directory to save downloaded files
    pub async fn save_dir(path: &str) {
        let is_save_set: bool;
        if path.starts_with("./") {
            is_save_set = env::current_dir().unwrap().as_path().ends_with(&path[2..]);
        } else {
            is_save_set = env::current_dir().unwrap().as_path().ends_with(path);
        }

        if !is_save_set {
            if let Err(e) = env::set_current_dir(path) {
                if e.kind() == ErrorKind::NotFound {
                    logger::log(&format!("Creating Directory: \"{}\"", path));
                } else {
                    eprintln!("{}", e);
                }
                fs::create_dir_all(path).await.expect(&*format!(
                    "directory for path, '{}', cannot be created!",
                    path
                ));
                env::set_current_dir(path).unwrap_or_else(|_e| {
                    env::set_current_dir(".").expect("Cannot set path as a save location!");
                });
            };
        }

        let x = env::current_dir().unwrap();
        logger::log_split("Save Directory", &format!("{}", x.display()));
    }
    /// Start stat operations
    async fn stat_tasks(
        stat: asset::stat::Stat,
        recorder_id: usize,
        file_name: &str,
        verbose: bool,
    ) {
        if verbose {
            logger::new_line();
            logger::log(&format!("{:?}", stat.extension_map));
        }

        let stats_file = format!(
            r"{}\{}_stats_{}",
            env::current_dir().unwrap().to_string_lossy(),
            recorder_id,
            file_name
        );
        if let Err(e) = Recorder::create_stats_file(stat, &stats_file).await {
            eprintln!("Cannot make stat file. {}", e);
        }
    }
    // /Create a JSON file with stats about each URL recorded
    async fn create_stats_file(stat: asset::stat::Stat, record_path: &str) -> Result<(), Error> {
        let mut f = fs::File::create(record_path).await?;
        Recorder::json_writer(&mut f, stat).await
    }
    async fn json_writer(f: &mut fs::File, mut stat: asset::stat::Stat) -> Result<(), Error> {
        stat.sort_files();
        let json = serde_json::to_string_pretty(&stat)?;
        f.write_all(json.as_bytes()).await
    }
}
