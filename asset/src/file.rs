use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::PartialEq;
use url::Url;
use urlencoding;

lazy_static! {
    static ref LAST_SLASH_REG: Regex = Regex::new(r"/$").unwrap();
    static ref FILE_EXT_REG: Regex =
        Regex::new(r"/.+\.(?:[a-zA-Z0-9]{3,7}|[a-zA-Z][a-zA-Z0-9]|[0-9][a-zA-Z])/?$").unwrap();
    static ref QUERY_REG: Regex = Regex::new(r"/\?\w+=\w+/").unwrap();
    static ref QUERY_PATH_REG: Regex = Regex::new(r"/\?/").unwrap();
}
#[derive(Debug)]
pub struct File {
    pub link: String,
    pub name: String,
    pub short_name: Option<String>,
    pub ext: Option<String>,
    pub dir_path: String,
}
impl PartialEq for File {
    fn eq(&self, other: &Self) -> bool {
        self.link == other.link
    }
    fn ne(&self, other: &Self) -> bool {
        self.link != other.link
    }
}
impl File {
    pub fn new(link: &str) -> File {
        let mut name = File::retrieve_name(link).unwrap_or(String::from("untitled"));
        name = File::decode_entities(&name);
        let ext = File::part_of_name(&name, true);
        let short_name = File::part_of_name(&name, false);
        let mut dir_path = File::dir_path(link);
        dir_path = File::decode_entities(&dir_path);
        File {
            link: link.to_string(),
            name,
            ext,
            short_name,
            dir_path,
        }
    }
    /// Retrieve the parent path(directories) of the file
    fn dir_path(link: &str) -> String {
        let no_query_url = QUERY_PATH_REG.replace(link, "/");
        let url = Url::parse(no_query_url.as_ref()).unwrap();

        let path = url.path();

        let dir_path = FILE_EXT_REG.replace(path, "/");

        dir_path.to_string()
    }
    /// Removes a path that starts a question mark. '/?/'
    fn query_check(url: &str) -> Option<String> {
        if QUERY_REG.is_match(url) {
            let new_url = QUERY_REG.replace(url, "/");
            let url = Url::parse(new_url.as_ref()).unwrap();
            let paths = url.path_segments().unwrap();
            let path = paths.last().unwrap();
            Some(path.to_string())
        } else {
            None
        }
    }
    /// Retrieve the name of the file from the URL
    fn retrieve_name(url: &str) -> Option<String> {
        if let Some(name) = File::query_check(url) {
            return Some(name);
        }

        let mut mut_url = Url::parse(url).unwrap();
        let immut_url = Url::parse(url).unwrap();

        if mut_url.path() == "/" {
            if let Some(query) = immut_url.query() {
                if query.starts_with("/") {
                    mut_url.set_query(None);
                    mut_url.set_path(query);
                } else {
                    return None;
                }
            }
        }
        let no_end_slash = LAST_SLASH_REG.replace(mut_url.path(), "").to_string();
        mut_url.set_path(no_end_slash.as_str());
        let url_paths = mut_url
            .path_segments()
            .ok_or_else(|| "cannot as base")
            .unwrap();

        match url_paths.last() {
            Some(name) => {
                if !name.is_empty() {
                    if name.ends_with("/") {
                        let mut name = String::from(name);
                        name.remove(name.len() - 1);
                        Some(name)
                    } else {
                        Some(name.to_string())
                    }
                } else {
                    None
                }
            }
            None => None,
        }
    }
    /// Get the file extension
    pub fn part_of_name(name: &str, get_ext: bool) -> Option<String> {
        let name_split: Vec<&str> = name.split('.').collect();
        if name_split.len() >= 2 {
            if get_ext {
                Some(String::from(name_split[name_split.len() - 1]))
            } else {
                Some(String::from(name_split[name_split.len() - 2]))
            }
        } else {
            None
        }
    }
    /// Decode URL entities
    fn decode_entities(x: &str) -> String {
        match urlencoding::decode(x) {
            Ok(decoded_msg) => decoded_msg.into_owned(),
            Err(_e) => x.to_string(),
        }
    }
    pub fn to_json(self) -> String {
        match self.short_name {
            Some(name) => format!("{}.json", name),
            None => format!("{}.json", self.name),
        }
    }
}

#[cfg(test)]
mod test {
    use super::FILE_EXT_REG;
    fn file_regex_pass(x: &str) {
        assert_eq!(FILE_EXT_REG.is_match(x), true);
    }
    fn file_regex_fail(x: &str) {
        assert_ne!(FILE_EXT_REG.is_match(x), true);
    }
    #[test]
    fn file_test() {
        file_regex_pass("https://api.example.com\
        .com/Best%20Song%20Contest%202013/Ach%20nein,%20es%20ist%20M%fcll%20-%20Deponia%20Original-Soundtrack.mp3");

        file_regex_pass(
            "https://api.example.com/Best%20Song%20Contest%202013/Ach%20nein,\
        %20es%20ist%20M%fcll%20-%20Deponia%20Original-Soundtrack.mp3/",
        );

        file_regex_pass("https://api.example.com/Best%20Song%20Contest%202013/Adder's%20Lair%20%5bWilderness%20~%20Turtle%20Village%201%20-%20Golden%20Axe%5d%20-%20Sonic%20&%20All-Stars%20Racing%20Transformed%20-%20Original%20Sound%20Version.mp3");

        file_regex_pass(
            "https://api.example.com/Best%20Song%20Contest%202013/Soundtrack%20%23002%20\
        -%20Heaven%20Variant.mp3",
        );

        file_regex_fail(
            "https://api.example.com/Best%20Song%20Contest%202013/Soundtrack%20%23002%20\
        -%20Heaven%20Variant",
        );

        file_regex_fail(
            "https://api.example.com/Best%20Song%20Contest%202013/Soundtrack%20%23002%20\
        -%20Heaven%20Variant/",
        );

        file_regex_fail("https://api.example.com/Best%20Song%20Contest%202013/");
        file_regex_fail("https://api.example.com/Best%20Song%20Contest%202013");
        file_regex_pass("https://api.example.com/Best%20Song%20Contest%202013.7z");
    }
}
