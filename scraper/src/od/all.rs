use super::directory_listing_script::DirectoryListingScript;
use super::microsoftiis;
use super::olaindex::{OlaindexExtras, OLAINDEX};
use crate::parser;
use select::document::Document;
use select::predicate::Name;
/// Parses the usual HTML Document type ods
pub fn search(res: &str, url: &str) -> Vec<String> {
    Document::from(res)
        //Find all <a> tags
        .find(Name("a"))
        .filter(|node| no_parent_dir(url, &node.text(), node.attr("href")))
        .filter_map(|node| node.attr("href"))
        .filter(|link| {
            let mut paths: Vec<&str> = link.split("/").collect();
            !OLAINDEX::has_extra_paths(&mut paths, OlaindexExtras::ExcludeHomeAndDownload)
        })
        .filter(|link| !parser::ends_with_any_query(link))
        .filter(|link| !link.contains("javascript:"))
        .map(|link| parser::sanitize_url(link))
        .collect()
}
/// Check if link leads back to parent directory
pub fn no_parent_dir(url: &str, content: &str, href: Option<&str>) -> bool {
    let trimmed_content = content.trim();
    let content = trimmed_content.to_lowercase();
    let back_paths = vec![".", "../", "..", "./"];
    //Check for `parent directory` phrase
    let not_parent_dir = (!content.starts_with("parent directory")
        && !content.starts_with("Parent Directory"))
        && content != microsoftiis::IDENTIFIER.to_lowercase();
    //Check for back paths in href
    let no_back_path_in_href = match href {
        Some(link) => !back_paths.iter().any(|back| back == &link),
        None => false,
    };

    //Check for `www.example.com/index.php?dir=`
    let no_home_navigator = match href {
        Some(link) => !DirectoryListingScript::is_home_navigator(link),
        None => false,
    };
    //Check for URLs leading back to homepage
    let no_home_url = match href {
        Some(link) => {
            let mut new_link = parser::remove_http(link);
            new_link = parser::remove_last_slash(&new_link);
            let mut new_url = parser::remove_http(url);
            new_url = parser::remove_last_slash(&new_url);
            new_link != new_url
        }
        None => false,
    };
    not_parent_dir && no_back_path_in_href && no_home_navigator && no_home_url
}

#[cfg(test)]
mod tests {
    use super::no_parent_dir;

    #[test]
    fn no_parent_test() {
        const HOME_URL: &str = "https://ftp.example.jp";
        assert_eq!(
            no_parent_dir(HOME_URL, "Parent directory/", Some("../")),
            false
        );
        assert_eq!(
            no_parent_dir(HOME_URL, "Parent Directory", Some("..")),
            false
        );
        assert_eq!(
            no_parent_dir(HOME_URL, "Parent Directory", Some("./")),
            false
        );
        assert_eq!(
            no_parent_dir(HOME_URL, "parent directory", Some(".")),
            false
        );
        assert_eq!(
            no_parent_dir(HOME_URL, "Carrots and java", Some("../")),
            false
        );
        assert_eq!(
            no_parent_dir(HOME_URL, "Carrots and java", Some("./")),
            false
        );
        assert_eq!(
            no_parent_dir(HOME_URL, "Carrots and java", Some("..")),
            false
        );
        assert_eq!(
            no_parent_dir(HOME_URL, "Carrots and java", Some(".")),
            false
        );
        assert_eq!(
            no_parent_dir(
                HOME_URL,
                "Carrots and java",
                Some("https://www.example.com/path/index.php?dir=")
            ),
            false
        );
        assert_eq!(
            no_parent_dir(HOME_URL, "Drink Soda", Some("https://ftp.example.jp")),
            false
        );
        assert_eq!(
            no_parent_dir(HOME_URL, "../", Some("https://ftp.example.jp")),
            false
        );
        assert_eq!(
            no_parent_dir(HOME_URL, "..", Some("https://ftp.example.jp")),
            false
        );
        assert_eq!(
            no_parent_dir(HOME_URL, "./", Some("https://ftp.example.jp")),
            false
        );
        assert_eq!(
            no_parent_dir(HOME_URL, ".", Some("https://ftp.example.jp")),
            false
        );

        assert_eq!(
            no_parent_dir(HOME_URL, "Carrots and java", Some("./Carrots%20and%20java")),
            true
        );
        assert_eq!(
            no_parent_dir(HOME_URL, "../Carrots", Some("./Carrots%20and%20java")),
            true
        );
        assert_eq!(
            no_parent_dir(HOME_URL, "Drink Soda", Some("Drink%20Soda")),
            true
        );
        assert_eq!(
            no_parent_dir(
                HOME_URL,
                "Carrots and java",
                Some("https://www.example.com/path/index.php?dir=Outboards%2F5-27")
            ),
            true
        );
        assert_eq!(
            no_parent_dir(HOME_URL, "Drink Soda", Some("https://example.me")),
            true
        );
    }
}
