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
    opts.record = true; // Activate Recorder
    opts.depth = 1; // Current directory only
    opts.output = PathBuf::from("./examples/all"); // Location to save files
    let url: PathBuf = PathBuf::from("https://herooneindex.herokuapp.com/E5/"); // OD(s)
    opts.urls = vec![url];
}
