use cmd_opts::Opts;
use std::path::PathBuf;
use zeiver::Zeiver;

#[tokio::main]
async fn main() {
    let mut opts: Opts = Opts::new();
    init_opts(&mut opts);
    Zeiver::start(opts).await;
}

/// Setup commandline options
fn init_opts(opts: &mut Opts) {
    opts.print_headers = true; // Print all Response headers
    let url_one: PathBuf = PathBuf::from("https://herooneindex.herokuapp.com/E5/"); // OD
    let url_two: PathBuf = PathBuf::from("https://demo.directorylister.com/"); // OD
    let url_three: PathBuf = PathBuf::from("https://od.xbottle.top"); // OD
    opts.urls = vec![url_one, url_two, url_three];
}
