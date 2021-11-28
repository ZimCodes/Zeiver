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
    opts.depth = 1; // Current directory only
    opts.cut_dirs = 3; // Removes the creation of directories 3 levels deep
    opts.output = PathBuf::from("./examples/no_recording"); // Location to save files
    let url: PathBuf = PathBuf::from("https://od.xbottle.top/Previews/Audios/"); // OD(s)
    opts.urls = vec![url];
}
