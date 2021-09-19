use select::document::Document;
use select::predicate::{Name, Class, Predicate};
use crate::parser;
use crate::od::olaindex::{OLAINDEX, OlaindexExtras};
use crate::od::ODMethod;
use select::node::Node;

/// Parses the Directory Lister HTML Document type ods
fn directory_lister_document(res: &str, url: &str) -> Vec<String> {
    Document::from(res)
        //Find all <a> tags
        .find(Name("ul").descendant(Name("a")))
        .filter(|node| {
            let link = node.attr("href").unwrap();
            !url.contains(link) && no_parent_dir(node)
        })
        .filter_map(|node| {
            node.attr("href")
        }).filter(|link| !link.contains("javascript:void"))
        .map(|link| parser::sanitize_url(link)).collect()
}

/// Parses the AutoIndex PHP HTML Document type ods
fn autoindex_document(res: &str) -> Vec<String> {
    Document::from(res)
        //Find all <a> tags
        .find(Name("tbody").descendant(Class("autoindex_a").or(Class("default_a"))))
        .filter(|node| no_parent_dir(node))
        .filter_map(|node| {
            node.attr("href")
        }).filter(|link| !link.contains("javascript:void"))
        .map(|link| parser::sanitize_url(link)).collect()
}

/// Parses the OLAINDEX HTML Document type ods
fn olaindex_document(res: &str) -> Vec<String> {
    Document::from(res)
        //Find all <a data-route> tags
        .find(Name("div")
            .and(Class("mdui-container").or(Class("container")))
            .descendant(Name("a").or(Name("li"))))
        .filter(|node| no_parent_dir(node))
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
        }).filter(|link| !link.contains("javascript:void"))
        .map(|link| parser::sanitize_url(link)).collect()
}

/// Parses the usual HTML Document type ods
fn generic_document(res: &str) -> Vec<String> {
    Document::from(res)
        //Find all <a> tags
        .find(Name("a"))
        .filter(|node| no_parent_dir(node))
        .filter_map(|node| {
            node.attr("href")
        }).filter(|link| {
        let mut paths: Vec<&str> = link.split("/").collect();
        !OLAINDEX::has_extra_paths(&mut paths, OlaindexExtras::ExcludeHomeAndDownload)
    }).filter(|link| !link.contains("javascript:void"))
        .map(|link| parser::sanitize_url(link)).collect()
}

/// Switch to a different way to parse Document type
pub fn filtered_links(res: &str, url: &str, od_type: &ODMethod) -> Vec<String> {
    match od_type {
        ODMethod::OLAINDEX => olaindex_document(res),
        ODMethod::AutoIndexPHP | ODMethod::AutoIndexPHPNoCrumb => autoindex_document(res),
        ODMethod::DirectoryLister => directory_lister_document(res, url),
        _ => generic_document(res)
    }
}

/// Check if link leads back to parent directory
fn no_parent_dir(node: &Node) -> bool {
    let not_parent_dir = node.text().trim().to_lowercase() != "parent directory";
    let no_back_path = match node.attr("href") {
        Some(link) => link != ".",
        None => false
    };
    not_parent_dir && no_back_path
}