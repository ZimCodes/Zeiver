use lazy_static::lazy_static;
use regex::Regex;
use std::path::MAIN_SEPARATOR;

lazy_static! {
    static ref NOT_UNIX_LINES: Regex = Regex::new(r"/{2,}|\\+").unwrap();
    static ref NOT_WINDOWS_LINES: Regex = Regex::new(r"\\{2,}|/+").unwrap();
}

/// Get EOL based on operating system
pub fn get_line_separator() -> String {
    if is_windows() {
        String::from("\r\n")
    } else {
        String::from("\n")
    }
}

/// Corrects file paths in different environments.
/// Dependent on current working operating system
pub fn correct_os_path(path: &str) -> String {
    if path.starts_with("./") {
        return path.to_string();
    }
    if is_windows() {
        NOT_WINDOWS_LINES
            .replace_all(path, MAIN_SEPARATOR.to_string())
            .to_string()
    } else if NOT_UNIX_LINES.is_match(path) {
        NOT_UNIX_LINES
            .replace_all(path, MAIN_SEPARATOR.to_string())
            .to_string()
    } else {
        path.to_string()
    }
}

/// Check if operating system is windows
pub fn is_windows() -> bool {
    MAIN_SEPARATOR != '/'
}

#[cfg(test)]
mod tests {
    use super::{correct_os_path, is_windows, NOT_UNIX_LINES, NOT_WINDOWS_LINES};
    fn is_corrected_path(path: &str, correct: &str) {
        let is_windows = is_windows();
        let guess = correct_os_path(path);
        let is_windows_problem = correct.contains(r"\");
        if (is_windows_problem && is_windows)
            || (!is_windows_problem && !is_windows)
            || guess == correct
        {
            assert_eq!(guess, correct);
        } else {
            assert_ne!(correct_os_path(path), correct);
        }
    }
    #[test]
    fn path_correction() {
        is_corrected_path(r"/etc/passwd", r"/etc/passwd");
        is_corrected_path(r"passwd", r"passwd");
        is_corrected_path(r"etc\passwd", r"etc/passwd");
        is_corrected_path(r"/etc/passwd///////", r"/etc/passwd/");
        is_corrected_path(r".\etc\passwd", r"./etc/passwd");
        is_corrected_path(r"./etc\passwd", r"./etc/passwd");
        is_corrected_path(
            r"C:\Users\Person\Documents\Zeiver\scraper\cat\bonjour.jpg",
            r"C:\Users\Person\Documents\Zeiver\scraper\cat\bonjour.jpg",
        );
        is_corrected_path(
            r"C:\\Users\Person\Documents\Zeiver\scraper\cat\bonjour.jpg",
            r"C:\Users\Person\Documents\Zeiver\scraper\cat\bonjour.jpg",
        );
        is_corrected_path(
            r"C:\Users\Person\Documents\Zeiver\scraper\cat/bonjour.jpg",
            r"C:\Users\Person\Documents\Zeiver\scraper\cat\bonjour.jpg",
        );
        is_corrected_path(
            r"C:\\Users\\Person\Documents\\Zeiver\\scraper\\cat\\bonjour.jpg",
            r"C:\Users\Person\Documents\Zeiver\scraper\cat\bonjour.jpg",
        );
        is_corrected_path(
            r"C:\Users\Person\Documents\Zeiver\\\\\\\\\\\scraper\\cat\\bonjour.jpg",
            r"C:\Users\Person\Documents\Zeiver\scraper\cat\bonjour.jpg",
        );
        is_corrected_path(
            r"./Users/Person/Documents/Zeiver/scraper/cat/bonjour.jpg",
            r"./Users/Person/Documents/Zeiver/scraper/cat/bonjour.jpg",
        );
    }

    #[test]
    fn is_unix_path_regex() {
        assert_eq!(NOT_UNIX_LINES.is_match(r"/etc/passwd"), false);
        assert_eq!(NOT_UNIX_LINES.is_match(r"./etc/passwd"), false);
        assert_eq!(NOT_UNIX_LINES.is_match(r"/passwd"), false);
        assert_eq!(NOT_UNIX_LINES.is_match(r"passwd"), false);
        assert_eq!(NOT_UNIX_LINES.is_match(r"passwd/"), false);
        assert_eq!(NOT_UNIX_LINES.is_match(r"/etc/passwd/"), false);
    }

    #[test]
    fn not_unix_path_regex() {
        assert_eq!(
            NOT_UNIX_LINES.is_match(r"C:\Users\Person\Documents\Zeiver\scraper\cat\bonjour.jpg"),
            true
        );
        assert_eq!(NOT_UNIX_LINES.is_match(r"etc\passwd"), true);
        assert_eq!(NOT_UNIX_LINES.is_match(r"\etc\passwd"), true);
        assert_eq!(NOT_UNIX_LINES.is_match(r"passwd\"), true);
        assert_eq!(NOT_UNIX_LINES.is_match(r"/etc//passwd/"), true);
        assert_eq!(NOT_UNIX_LINES.is_match(r"/etc/passwd///////"), true);
        assert_eq!(NOT_UNIX_LINES.is_match(r"/etc\passwd"), true);
        assert_eq!(NOT_UNIX_LINES.is_match(r".\etc\passwd"), true);
        assert_eq!(NOT_UNIX_LINES.is_match(r"./etc\passwd"), true);
    }

    #[test]
    fn is_windows_path_regex() {
        assert_eq!(
            NOT_WINDOWS_LINES.is_match(r"C:\Users\Person\Documents\Zeiver\scraper\cat\bonjour.jpg"),
            false
        );
        assert_eq!(NOT_WINDOWS_LINES.is_match(r"passwd"), false);
    }

    #[test]
    fn not_windows_path_regex() {
        assert_eq!(NOT_WINDOWS_LINES.is_match(r"/etc/passwd"), true);
        assert_eq!(NOT_WINDOWS_LINES.is_match(r"./etc/passwd"), true);
        assert_eq!(NOT_WINDOWS_LINES.is_match(r"/passwd"), true);
        assert_eq!(NOT_WINDOWS_LINES.is_match(r"passwd/"), true);
        assert_eq!(NOT_WINDOWS_LINES.is_match(r"/etc/passwd/"), true);
        assert_eq!(NOT_WINDOWS_LINES.is_match(r"/etc/passwd/"), true);
        assert_eq!(
            NOT_WINDOWS_LINES
                .is_match(r"C:\\Users\Person\Documents\Zeiver\scraper\cat\bonjour.jpg"),
            true
        );
        assert_eq!(
            NOT_WINDOWS_LINES.is_match(r"C:\Users\Person\Documents\Zeiver\scraper\cat/bonjour.jpg"),
            true
        );
        assert_eq!(
            NOT_WINDOWS_LINES
                .is_match(r"C:\\Users\\Person\Documents\\Zeiver\\scraper\\cat\\bonjour.jpg"),
            true
        );
        assert_eq!(
            NOT_WINDOWS_LINES
                .is_match(r"C:\Users\Person\Documents\Zeiver\\\\\\\\\\\scraper\\cat\\bonjour.jpg"),
            true
        );

        assert_eq!(
            NOT_WINDOWS_LINES.is_match(r"./Users/Person/Documents/Zeiver/scraper/cat/bonjour.jpg"),
            true
        );
    }
}
