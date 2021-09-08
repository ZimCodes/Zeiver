use std::fmt;

pub struct Directory{
    pub link:String,
}
impl  Directory{
    pub fn new(link: String) -> Directory{
        Directory{
            link
        }
    }
}
impl fmt::Debug for Directory{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) ->fmt::Result{
        writeln!(f,"{}",self.link)
    }
}