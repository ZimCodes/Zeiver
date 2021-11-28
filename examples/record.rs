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
    opts.verbose = true; // Reveal all progress in terminal
    opts.record_only = true; // Use Scraper & Recorder only
    opts.depth = 3; // Directories deep
    opts.output = PathBuf::from("./examples/record"); // Location to save files
    let url: PathBuf = PathBuf::from("https://demo.directorylister.com/"); // OD(s)
    opts.urls = vec![url];
}
