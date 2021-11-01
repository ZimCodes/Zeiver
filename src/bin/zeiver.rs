use cmd_opts::Opts;
use zeiver::Zeiver;
#[tokio::main]
async fn main() {
    let opts = Opts::new();
    Zeiver::start(opts).await;
}
