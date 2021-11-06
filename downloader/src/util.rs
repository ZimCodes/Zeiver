use bytes::Bytes;
use lazy_static::lazy_static;
use regex::Regex;
use std::env;
use tokio::fs;
use tokio::io::{AsyncWriteExt, ErrorKind};

use asset;
use compat;
use logger;

lazy_static! {
    static ref ONE_PATH_REG: Regex =
        Regex::new(r"/[a-zA-Z0-9\*~\+\-%\?\[\]\$_\.!‘\(\)=]+/").unwrap();
}
pub enum HttpBodyType {
    Text(String),
    Binary(Bytes),
}
/// Prepares the File for download
pub async fn prepare_file(
    res_content: Box<HttpBodyType>,
    file: &asset::file::File,
    cuts: u32,
    use_dir: bool,
) {
    let f = create_file_path(file, cuts, use_dir).await;
    match *res_content {
        HttpBodyType::Text(text) => {
            let file_byte = text.as_bytes();
            logger::head(&format!("Downloading {}", file.name));
            download_progress(f, file_byte, file.name.as_str()).await;
        }
        HttpBodyType::Binary(data) => {
            let file_byte = data.as_ref();
            logger::head(&format!("Downloading {}", file.name));
            download_progress(f, file_byte, file.name.as_str()).await;
        }
    };
}
/// Downloads the file while showing its current progress
pub async fn download_progress(mut f: fs::File, file_byte: &[u8], name: &str) {
    let file_length = file_byte.len();
    let mut data_length: usize = 0;

    while let Ok(byte) = retrieve_buffer_data(&mut f, file_byte).await {
        data_length += byte;
        if data_length >= file_length {
            break;
        }
    }
    logger::info("File Size", &byte_calc(file_length));
    logger::info("Downloaded", &byte_calc(data_length));
    logger::head(&format!("{} Downloaded!", name));
}
async fn retrieve_buffer_data(f: &mut fs::File, file_byte: &[u8]) -> Result<usize, ()> {
    match f.write(file_byte).await {
        Ok(byte) => Ok(byte),
        Err(e) => match e.kind() {
            ErrorKind::Interrupted => Ok(0usize),
            _ => {
                eprintln!("No bytes in the buffer were written to this File");
                Err(())
            }
        },
    }
}
/// Byte formatting
fn byte_calc(total: usize) -> String {
    let units: [&str; 9] = ["B", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];
    let total: f32 = total as f32;
    let mut index: usize = 0;
    let mut amount: f32 = total;
    while amount > 1024.00 {
        amount = amount / 1024.00;
        index += 1;
    }

    let size = format!("{:.2}{}", amount, units[index]);
    size
}
/// Creates a file path
async fn create_file_path(file: &asset::file::File, cuts: u32, use_dir: bool) -> fs::File {
    let cur_dir = env::current_dir().expect("Current directory cannot be retrieved");
    let cur_dir = match cur_dir.to_str() {
        Some(dir) => dir,
        None => "./",
    };
    let save_dir_path = link_dir_path(file, cur_dir, cuts, use_dir).await;

    let save_file_path = file_path_join(file, save_dir_path.as_str());
    logger::new_line();
    logger::log_split("SAVE FILE PATH", &save_file_path);
    logger::new_line();
    let f = match fs::File::create(save_file_path).await {
        Ok(f) => f,
        Err(e) => panic!("File cannot be created! Reason: {}", e.to_string()),
    };

    f
}
/// Joins save location with file name to create a path
fn file_path_join(file: &asset::file::File, save_dir_path: &str) -> String {
    if !file.name.starts_with("/")
        && !file.name.starts_with(r"\")
        && !save_dir_path.ends_with(r"\")
        && !save_dir_path.ends_with(r"/")
    {
        compat::correct_os_path(&*format!(r"{}\{}", save_dir_path, file.name))
    } else {
        compat::correct_os_path(&*format!(r"{}{}", save_dir_path, file.name))
    }
}
/// Link remote directory path with local save location
async fn link_dir_path(
    file: &asset::file::File,
    cur_dir: &str,
    cuts: u32,
    use_dir: bool,
) -> String {
    if use_dir {
        let dir_path = format!("{}{}", cur_dir, cut_dir(file, cuts).replace(":", ""));
        if let Err(e) = fs::create_dir_all(&dir_path).await {
            match e.kind() {
                ErrorKind::AlreadyExists => logger::log(&format!("{} already exists!", dir_path)),
                _ => panic!("{}", e.to_string()),
            }
        }
        dir_path
    } else {
        cur_dir.to_string()
    }
}
/// Remove specified amount of directories from remote URL
fn cut_dir(file: &asset::file::File, cuts: u32) -> String {
    if cuts != 0 {
        let path = file.dir_path.as_str();
        let mut path = String::from(path);
        for _ in 0..cuts {
            path = ONE_PATH_REG.replace(path.as_str(), "/").to_string();
        }
        path
    } else {
        let x = file.dir_path.as_str();
        String::from(x)
    }
}
