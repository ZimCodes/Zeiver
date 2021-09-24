use colored::*;

/// Prints a declarative info text to terminal
pub fn info(name: &str, stat: &str) {
    let line = "--->|".bright_magenta();
    let other_line = "|<---".bright_magenta();
    let name = name.bright_yellow();
    let semi_colon = ":".bright_yellow();
    let stat = stat.bright_cyan().bold();
    log(&format!("{} {}{} {} {}", line, name, semi_colon, stat, other_line));
}

/// star formatted header
pub fn stars_info(name: &str, stat: &str) {
    let line = "***".bright_magenta();
    let name = format!("{}:", name);
    let info = stat.bright_cyan().bold();
    log(&format!("{} {} {} {}", line, name.as_str().bright_yellow(), info, line));
}
/// Double horizontal divider
pub fn divider(){
    println!("==================");
}
/// Prints a formatted header to the terminal
pub fn head(title: &str) {
    let line = "-----".bright_magenta();
    log(&format!("{}{}{}", line, title.bright_yellow().bold().underline(), line));
}

/// Arrow formatted header
pub fn arrows_head(text: &str) {
    let line = "----->".bright_magenta();
    let other_line = "<-----".bright_magenta();
    let text = text.bright_yellow().bold().underline();
    log(&format!("{}  {}  {}", line, text, other_line));
    new_line();
}

/// Creates a new line break
pub fn new_line() {
    log("");
}

pub fn log_split(name:&str,val:&str){
    let name = format!("{}:",name);
    let value = val.bright_cyan();
    log(&format!("{} {}",name.as_str().bold().bright_yellow(),value));
}
/// Prints message to terminal
pub fn log(message: &str) {
    println!("{}", message);
}
pub fn log_underline(text:&str){
    let text = text.bright_yellow().bold().underline();
    log(&text);
}
pub fn error(text:&str){
    let text = text.bright_red().italic();
    log(&text);
}