use crate::od::directory_listing_script::DirectoryListingScript;
use crate::od::lighttpd::LightTPD;
use crate::od::microsoftiis;
use crate::od::nginx::NGINX;
use crate::od::olaindex::{OlaindexExtras, OLAINDEX};
use crate::od::phpbb::PHPBB;
use crate::od::snif;
use crate::od::ODMethod;
use crate::parser;
use select::document::Document;
use select::predicate::{Attr, Class, Name, Not, Predicate};
/// Parses the PanIndex type ods
fn panindex_document(res: &str, url: &str) -> Vec<String> {
    Document::from(res)
        .find(Name("div").and(Attr("data-url", ())))
        .filter(|node| no_parent_dir(url, &node.text(), node.attr("data-url")))
        .filter_map(|node| node.attr("data-url"))
        .filter(|link| !link.contains("javascript:"))
        .map(|link| parser::sanitize_url(link))
        .collect()
}
/// Parses OdIndex HTML Documents
fn odindex_documents(res: &str, url: &str) -> Vec<String> {
    Document::from(res)
        .find(Name("a").and(Class("item")))
        .filter(|node| no_parent_dir(url, &node.text(), node.attr("href")))
        .filter(|node| !node.text().ends_with(".."))
        .filter_map(|node| node.attr("href"))
        .filter(|link| !link.contains("javascript:"))
        .map(|link| parser::sanitize_url(link))
        .collect()
}
/// Parses Snif HTML Documents
fn snif_documents(res: &str, url: &str) -> Vec<String> {
    Document::from(res)
        .find(
            Name("tr")
                .and(Class("snF"))
                .descendant(Name("td"))
                .descendant(Name("a")),
        )
        .filter(|node| no_parent_dir(url, &node.text(), node.attr("href")))
        .filter(|node| !snif::Snif::is_parent(node.attr("title")))
        .filter(|node| !snif::Snif::is_download(node.attr("href")))
        .filter_map(|node| node.attr("href"))
        .filter(|link| !link.contains("javascript:"))
        .map(|link| parser::sanitize_url(link))
        .collect()
}
/// Parses Microsoft-IIS HTML Documents
fn microsoft_iis_documents(res: &str, url: &str) -> Vec<String> {
    Document::from(res)
        .find(Name("pre").descendant(Name("a")))
        .filter(|node| no_parent_dir(url, &node.text(), node.attr("href")))
        .filter_map(|node| node.attr("href"))
        .filter(|link| !link.contains("javascript:"))
        .map(|link| parser::sanitize_url(link))
        .collect()
}
/// Parses h5ai HTMl Documents
fn h5ai_document(res: &str, url: &str) -> Vec<String> {
    Document::from(res)
        .find(Name("td").and(Class("fb-n")).descendant(Name("a")))
        .filter(|node| no_parent_dir(url, &node.text(), node.attr("href")))
        .filter_map(|node| node.attr("href"))
        .filter(|link| !link.contains("javascript:"))
        .map(|link| parser::sanitize_url(link))
        .collect()
}

/// Parses OneIndex related HTML Documents
fn one_manager_oneindex_document(res: &str, url: &str) -> Vec<String> {
    Document::from(res)
        .find(
            Name("div")
                .and(Class("mdui-container"))
                .descendant(
                    Name("li").descendant(
                        Name("a").and(
                            Not(Attr("title", "download")).and(Not(Attr("title", "直接下载"))),
                        ),
                    ),
                ),
        )
        .filter(|node| no_parent_dir(url, &node.text(), node.attr("href")))
        .filter(|node| !node.text().contains("arrow_"))
        .filter_map(|node| node.attr("href"))
        .filter(|link| !link.ends_with("/?/"))
        .filter(|link| !link.contains("javascript:"))
        .map(|link| parser::sanitize_url(link))
        .collect()
}

/// Parses Modern OneManager HTML Documents
fn one_manager_modern_document(res: &str, url: &str) -> Vec<String> {
    let links: Vec<String> = Document::from(res)
        .find(
            Name("td")
                .and(Class("file"))
                .descendant(Name("a").and(Not(Class("download")))),
        )
        .filter(|node| no_parent_dir(url, &node.text(), node.attr("href")))
        .filter(|node| !node.text().contains("arrow_"))
        .filter_map(|node| node.attr("href"))
        .filter(|link| !link.ends_with("/?/"))
        .filter(|link| !link.contains("javascript:"))
        .map(|link| parser::sanitize_url(link))
        .collect();
    if links.is_empty() {
        one_manager_oneindex_document(res, url)
    } else {
        links
    }
}

/// Parses phpBB HTML Documents
fn phpbb_document(res: &str, url: &str) -> Vec<String> {
    Document::from(res)
        .find(
            Name("tr")
                .descendant(Name("td").descendant(Name("a")))
                .or(Name("pre").descendant(Name("a")))
                .or(Name("li").descendant(Name("a"))),
        )
        .filter(|node| no_parent_dir(url, &node.text(), node.attr("href")))
        .filter(|node| !PHPBB::is_a_sort_query(node.attr("href").unwrap()))
        .filter(|node| !PHPBB::is_copy_file(&node.text()))
        .filter_map(|node| node.attr("href"))
        .filter(|link| !link.contains("javascript:"))
        .map(|link| parser::sanitize_url(link))
        .collect()
}

/// Parses lighttpd HTML Documents
fn lighttpd_document(res: &str, url: &str) -> Vec<String> {
    let full_names = LightTPD::full_file_name(res);
    Document::from(res)
        .find(Name("tr").descendant(Name("td").descendant(Name("a"))))
        .filter(|node| no_parent_dir(url, &node.text(), node.attr("href")))
        .filter_map(|node| {
            let href = node.attr("href").unwrap();
            let new_href = format!("{}/", href);
            if full_names.contains(&new_href) {
                Some(new_href)
            } else {
                Some(href.to_string())
            }
        })
        .filter(|link| {
            let mut paths: Vec<&str> = link.split("/").collect();
            !OLAINDEX::has_extra_paths(&mut paths, OlaindexExtras::ExcludeHomeAndDownload)
        })
        .filter(|link| !link.contains("javascript:"))
        .map(|link| parser::sanitize_url(&link))
        .collect()
}

/// Parses the Evoluted Directory Listing Script HTML Document type ods
fn directory_listing_script_document(res: &str, url: &str) -> Vec<String> {
    Document::from(res)
        .find(
            Attr("id", "listingcontainer")
                .descendant(Name("a"))
                .or(Class("table-container").descendant(Name("a"))),
        )
        .filter(|node| no_parent_dir(url, &node.text(), node.attr("href")))
        .filter_map(|node| node.attr("href"))
        .filter(|link| !link.contains("javascript:"))
        .map(|link| parser::sanitize_url(link))
        .collect()
}

/// Parses the Apache & NGINX HTML Document type ods
fn apache_document(res: &str, url: &str) -> Vec<String> {
    Document::from(res)
        .find(
            Name("tr")
                .descendant(Name("td").descendant(Name("a")))
                .or(Name("pre").descendant(Name("a")))
                .or(Name("li").descendant(Name("a"))),
        )
        .filter(|node| no_parent_dir(url, &node.text(), node.attr("href")))
        .filter(|node| !NGINX::has_extra_query(node.attr("href").unwrap()))
        .filter_map(|node| node.attr("href"))
        .filter(|link| !link.contains("javascript:"))
        .map(|link| parser::sanitize_url(link))
        .collect()
}

/// Parses the Directory Lister HTML Document type ods
fn directory_lister_document(res: &str, url: &str) -> Vec<String> {
    Document::from(res)
        //Find all <a> tags
        .find(Name("ul").descendant(Name("a")))
        .filter(|node| {
            let link = node.attr("href").unwrap();
            !url.contains(link) && no_parent_dir(url, &node.text(), node.attr("href"))
        })
        .filter_map(|node| node.attr("href"))
        .filter(|link| !link.contains("javascript:"))
        .map(|link| parser::sanitize_url(link))
        .collect()
}

/// Parses the AutoIndex PHP HTML Document type ods
fn autoindex_document(res: &str, url: &str) -> Vec<String> {
    Document::from(res)
        //Find all <a> tags
        .find(Name("tbody").descendant(Class("autoindex_a").or(Class("default_a"))))
        .filter(|node| no_parent_dir(url, &node.text(), node.attr("href")))
        .filter_map(|node| node.attr("href"))
        .filter(|link| !link.contains("javascript:"))
        .map(|link| parser::sanitize_url(link))
        .collect()
}

/// Parses the OLAINDEX HTML Document type ods
fn olaindex_document(res: &str, url: &str) -> Vec<String> {
    Document::from(res)
        //Find all <a data-route> tags
        .find(
            Name("div")
                .and(Class("mdui-container").or(Class("container")))
                .descendant(Name("a").or(Name("li"))),
        )
        .filter(|node| no_parent_dir(url, &node.text(), node.attr("href")))
        .filter_map(|node| {
            if node.attr("data-route").is_some() {
                node.attr("data-route")
            } else {
                node.attr("href")
            }
        })
        .filter(|link| {
            let mut paths: Vec<&str> = link.split("/").collect();
            !OLAINDEX::has_extra_paths(&mut paths, OlaindexExtras::ExcludeHomeAndDownload)
        })
        .filter(|link| !link.contains("javascript:"))
        .map(|link| parser::sanitize_url(link))
        .collect()
}

/// Parses the usual HTML Document type ods
fn generic_document(res: &str, url: &str) -> Vec<String> {
    Document::from(res)
        //Find all <a> tags
        .find(Name("a"))
        .filter(|node| no_parent_dir(url, &node.text(), node.attr("href")))
        .filter_map(|node| node.attr("href"))
        .filter(|link| {
            let mut paths: Vec<&str> = link.split("/").collect();
            !OLAINDEX::has_extra_paths(&mut paths, OlaindexExtras::ExcludeHomeAndDownload)
        })
        .filter(|link| !link.contains("javascript:"))
        .map(|link| parser::sanitize_url(link))
        .collect()
}

/// Switch to a different way to parse Document type
pub fn filtered_links(res: &str, url: &str, od_type: &ODMethod) -> Vec<String> {
    match od_type {
        ODMethod::OLAINDEX => olaindex_document(res, url),
        ODMethod::AutoIndexPHP | ODMethod::AutoIndexPHPNoCrumb => autoindex_document(res, url),
        ODMethod::DirectoryLister => directory_lister_document(res, url),
        ODMethod::DirectoryListingScript => directory_listing_script_document(res, url),
        ODMethod::PHPBB => phpbb_document(res, url),
        ODMethod::OneIndex => one_manager_oneindex_document(res, url),
        ODMethod::OneManager => one_manager_modern_document(res, url),
        ODMethod::H5AI => h5ai_document(res, url),
        ODMethod::MicrosoftIIS => microsoft_iis_documents(res, url),
        ODMethod::Snif => snif_documents(res, url),
        ODMethod::PanIndex => panindex_document(res, url),
        ODMethod::OdIndex => odindex_documents(res, url),
        ODMethod::LightTPD => lighttpd_document(res, url),
        ODMethod::Apache | ODMethod::NGINX => apache_document(res, url),
        _ => generic_document(res, url),
    }
}

/// Check if link leads back to parent directory
fn no_parent_dir(url: &str, content: &str, href: Option<&str>) -> bool {
    let content = content.trim().to_lowercase();
    let back_paths = vec![".", "../", "..", "./"];
    //Check for `parent directory` phrase
    let not_parent_dir = !content.starts_with("parent directory")
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
