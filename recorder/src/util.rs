pub fn get_line_separator() -> String {
    if cfg!(target_os = "windows") {
        String::from("\r\n")
    } else {
        String::from("\n")
    }
}
