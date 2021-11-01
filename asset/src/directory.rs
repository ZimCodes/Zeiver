use std::fmt;

pub struct Directory {
    pub link: String,
    pub level: usize,
}
impl Directory {
    pub fn new(link: String, level: usize) -> Directory {
        Directory { link, level }
    }
}
impl fmt::Debug for Directory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.link)
    }
}
