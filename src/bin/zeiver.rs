use zeiver::Zeiver;
use cmd_opts::Opts;
#[tokio::main]
async fn main() {
    let opts = Opts::new();
    Zeiver::start(opts).await;
}

