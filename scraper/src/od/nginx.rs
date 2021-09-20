const IDENTIFIER:&str = "nginx";
pub struct NGINX;
impl NGINX{
    pub fn is_od(server:&str) -> bool{
         server.contains(IDENTIFIER)
    }
}