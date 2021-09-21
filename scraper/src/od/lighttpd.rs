use select::document::Document;
use select::predicate::{Name,Predicate,Class};

const IDENTIFIER:&str = "lighttpd";
pub struct LightTPD;
impl LightTPD{
    pub fn is_od(res:&str,server_name:&str) -> bool{
        LightTPD::footer_id(res) || LightTPD::script_title_id(res) || LightTPD::server_name_check(server_name)
    }
    /// Look at the `div.foot` for the id
    fn footer_id(res:&str)->bool{
        Document::from(res).find(Class("foot"))
            .any(|node| node.text().to_lowercase().starts_with(IDENTIFIER))
    }
    /// Look at the `div.script_title` for the id
    fn script_title_id(res:&str)->bool{
        Document::from(res).find(Class("script_title"))
            .any(|node| node.text().to_lowercase().starts_with(IDENTIFIER))
    }
    /// find the OD id through the `Server` Response header
    fn server_name_check(server_name:&str) -> bool{
        server_name.contains(IDENTIFIER)
    }
    /// Gets the names of the file and directory with the `/` included
    pub fn full_file_name(res:&str)->Vec<String>{
        Document::from(res)
            .find(Name("tr").descendant(Name("td")))
            .filter_map(|node| {
                Some(node.text())
            }).collect()
    }
}