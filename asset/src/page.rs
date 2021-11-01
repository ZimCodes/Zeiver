use crate::file::File;

pub struct Page {
    pub files: Vec<File>,
}
impl Page {
    pub fn new(files: Vec<File>) -> Page {
        Page { files }
    }
}
