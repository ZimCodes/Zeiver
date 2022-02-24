use cmd_opts::Opts;
use std::path::PathBuf;
use zeiver::Zeiver;

#[tokio::main]
async fn main() {
    let mut opts: Opts = Opts::new();
    init_opts(&mut opts);
    Zeiver::start(opts).await;
}

fn init_opts(opts: &mut Opts) {
    opts.verbose = true; // Reveal all progress in terminal
    opts.output = PathBuf::from("./examples/download_only"); // Location to save files
    let url: PathBuf = PathBuf::from("https://od.xbottle.top/Previews/Texts/Hey.txt"); // OD(s)
    opts.urls = vec![url];
}
