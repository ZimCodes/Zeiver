use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug, Clone)]
#[structopt(
    name = "Zeiver",
    about = "Scrape, record, download & scout content from ODs."
)]
pub struct Opts {
    ///Update Zeiver to latest version.
    #[structopt(short = "U",long,conflicts_with_all(&["record-only", "record", "cuts", "no-dirs", "output", "no-stats",
    "depth", "timeout", "wait", "retry-wait", "random-wait", "tries", "redirects", "accept", "reject",
    "u", "headers", "proxy", "proxy-auth", "input-file", "urls", "test", "scan", "print-header", "print-headers",
    "all-certs", "https-only","verbose","download-only","output","output-record","input-record"]))]
    pub update: bool,
    ///Enable verbose output
    #[structopt(short, long)]
    pub verbose: bool,
    /// Run a quick scrape test
    ///
    /// Use the Scraper without activating the Recorder and Downloader.
    #[structopt(long, conflicts_with_all(&["record-only", "record", "cuts", "no-dirs", "output", "output-record",
    "no-stats", "input-record"]))]
    pub test: bool,
    /// Scan ODs
    ///
    /// Scan ODs displaying their content to the terminal. *A shortcut to activating*
    /// `--verbose` *&* `--test`.
    #[structopt(long,conflicts_with_all(&["test","verbose","record-only","download-only"]))]
    pub scan: bool,
    ///Prints all Response Headers to terminal
    ///
    ///Prints all available Response headers received from each url to the terminal. **This option
    /// takes precedence over all other options**
    #[structopt(long, conflicts_with = "print-header")]
    pub print_headers: bool,
    ///Prints a Response Header to terminal
    ///
    /// Prints a specified Response Header to the terminal for each url. **This option takes
    /// precedence over all
    /// other options**.
    #[structopt(long)]
    pub print_header: Option<String>,
    /// Prints the HTML Document to the terminal
    ///
    /// Prints the HTML Document of each URL to the terminal for viewing. Allows you to see in
    /// the eyes of Zeiver. **This option takes precedence over all other options**.
    #[structopt(long, conflicts_with_all(&["print-header", "print-headers"]))]
    pub print_pages: bool,
    /// Specify the maximum depth for recursive scraping
    ///
    /// This option is used to determine how far to look into a directory(ies) to retrieve files.
    #[structopt(short = "d", long, default_value = "20")]
    pub depth: usize,
    /// Do not create directories from URL (download)
    ///
    /// Do not create a hierarchy of directories structured the same as the URL
    /// the file came from. All files will be saved to the current output directory instead.
    /// *Only available when downloading.
    #[structopt(long, parse(from_flag = std::ops::Not::not))]
    pub no_dirs: bool,
    /// Ignores a set of remote directories from being created
    ///
    /// Ignores a specified number of remote directories from being created.
    /// *Only available when downloading.
    /// Ex:
    /// URL: www.example.org/pub/xempcs/other/pics
    /// Original Save: ./pub/xempcs/other/pics
    /// After 2 cuts: ./other/pics
    #[structopt(short, long = "cuts", default_value = "0")]
    pub cut_dirs: u32,
    /// Enables a request timeout (in secs)
    ///
    /// Adds a request timeout (in seconds). The timeout is applied from the time the request starts
    /// connecting until the response body has finished. '0' seconds means the connection will
    /// never timeout.
    #[structopt(short = "T", long, default_value = "40")]
    pub timeout: u64,
    /// Wait between each HTTP request for scraping
    ///
    /// Wait a specified number of seconds before sending each scraping request.
    #[structopt(short, long)]
    pub wait: Option<f32>,
    /// Wait between each HTTP request for download
    ///
    /// Wait a specified number of seconds before sending each download request.
    #[structopt(long)]
    pub wait_download: Option<f32>,
    /// Use Downloader only
    ///
    /// Use only the Downloader to download all resources from links provided by an input file or
    /// command line.
    #[structopt(long,conflicts_with_all(&["output-record","record","record-only"]))]
    pub download_only: bool,
    /// Save the links only
    ///
    /// After scraping, instead of downloading the files, save the links to them.
    /// *The downloader will be disabled when this option is active. Enables
    /// Recorder instead.
    #[structopt(long)]
    pub record_only: bool,
    /// Activates the Recorder
    ///
    /// Enables the Recorder which saves the scraped links to a file. Disabled by default.
    /// *Option cannot be used with `--record-only`.
    #[structopt(long, conflicts_with = "record-only")]
    pub record: bool,
    /// Prevents Recorder from creating stat files
    ///
    /// The Recorder will no longer create stat files when saving scraped
    /// links to a file.
    #[structopt(long)]
    pub no_stats: bool,
    /// Prevent Recorder from writing file names to stat files
    ///
    /// Stat files includes the names of all files in alphabetical order
    /// alongside the number of file extensions. This option prevents the Recorder from adding file names
    /// to stat files.
    #[structopt(long, conflicts_with = "no-stats")]
    pub no_stats_list: bool,
    /// The wait between each failed request (secs)
    ///
    /// Whenever a request fails, Zeiver will wait the specified
    /// number of seconds before retrying again
    #[structopt(long, default_value = "8")]
    pub retry_wait: f32,
    /// Wait a random amount of seconds before each HTTP request for scraping
    ///
    /// Randomly waits a specified number of seconds before each scraping
    /// requests. The time between requests will vary between
    /// 0.5 * [--wait,-w](inclusive) to 1.5 * [--wait,-w](exclusive)
    #[structopt(long)]
    pub random_wait: bool,
    /// Wait a random amount of seconds before each HTTP request for download
    ///
    /// Randomly waits a specified number of seconds before each
    /// download request. The time between requests will vary between
    /// 0.5 * [--wait-download](inclusive) to 1.5 * [--wait-download](exclusive)
    #[structopt(long)]
    pub random_download: bool,
    /// The amount of times to retry a failed connection/request
    #[structopt(short, long, default_value = "20")]
    pub tries: u32,
    /// Maximum redirects to follow
    #[structopt(short, long, default_value = "10")]
    pub redirects: usize,
    /// Files to accept for scraping, downloading, & recording (Regex)
    ///
    /// Using Regex, specify which files to accept for downloading & recording.
    /// Only the files that matches the regex will be acceptable
    /// for scraping, downloading, & recording. (This option takes precedence over --reject, -R)
    #[structopt(short = "A", long)]
    pub accept: Option<String>,
    /// Files to reject for scraping, downloading, & recording (Regex)
    ///
    /// Using Regex, specify which files to reject for scraping, downloading, & recording.
    /// Only the files that match the regex will be rejected
    /// for scraping, downloading, & recording. (--accept, -A takes precedence over this option)
    #[structopt(short = "R", long, conflicts_with = "accept")]
    pub reject: Option<String>,
    /// The User Agent header to use
    #[structopt(short)]
    pub user_agent: Option<String>,
    /// Use HTTPS only
    ///
    /// Restrict Zeiver to send all requests through HTTPS connections only.
    #[structopt(long)]
    pub https_only: bool,
    /// Sets the default headers 'header:value'
    ///
    /// Sets the default headers for every request. Must use the
    /// 'header$value' format. Each header must also be separated by a comma.
    /// Ex: -H content-length$128,"accept$ text/html, application/xhtml+xml, image/webp"
    #[structopt(short = "H", long, use_delimiter(true))]
    pub headers: Option<Vec<String>>,
    /// The proxy to use
    #[structopt(long)]
    pub proxy: Option<String>,
    /// Authentication for the proxy 'username:password'
    ///
    /// The basic authentication needed to use the proxy. Must use the
    /// 'username:password' format.
    #[structopt(long)]
    pub proxy_auth: Option<String>,
    /// Accept all certificates *(Beware!)*
    ///
    /// Accept all certificates even invalid ones. **Use this option at your own risk!**
    #[structopt(long)]
    pub all_certs: bool,
    /// Read URLs from a local or external file
    ///
    /// Read URLs from a file to be sent to the Scraper. *Each line represents a URL to an OD.
    #[structopt(short, long, requires_ifs(&[("None", "urls"), ("None", "input-record")]))]
    pub input_file: Option<PathBuf>,
    /// Read URLs from a file containing file paths and create a stats file.
    ///
    /// Read URLs from an input file which contains links to other files and create a stats file
    /// based on the results. This option is for those who have a file filled with random
    /// unorganized links to a bunch of other files and want to take advantage of Zeiver's
    /// *Recorder* module.
    /// *Each line represents a URL to a file. **Activates Recorder**. Valid with `--verbose`,
    ///`--output`, `--output-record`, `--no-stats-list`
    #[structopt(long, conflicts_with_all(&["record-only", "record", "cuts", "no-dirs", "output", "no-stats",
    "depth", "timeout", "wait", "retry-wait", "random-wait", "tries", "redirects", "accept", "reject",
    "U", "u", "headers", "proxy", "proxy-auth", "input-file", "urls", "test", "scan", "print-header", "print-headers",
    "all-certs", "https-only"]))]
    pub input_record: Option<PathBuf>,
    /// Save Directory
    ///
    /// The local directory path to save files. Files saved by the *Recorder* are also stored here.
    /// Ex: ./downloads/images/dir
    #[structopt(short, long, default_value = "./")]
    pub output: PathBuf,
    /// Name of record file
    ///
    /// The name of the file to record the links received by the Recorder
    /// Ex: Link_file.txt
    #[structopt(long, default_value = "URL_Records.txt")]
    pub output_record: String,
    /// The URLs to download content from
    #[structopt(name = "URLS", requires_ifs(& [("None", "input-file"), ("None", "input-record")]))]
    pub urls: Vec<PathBuf>,
}

impl Opts {
    pub fn new() -> Opts {
        let mut opts = Opts::from_args();
        if opts.scan {
            opts.verbose = true;
            opts.test = true;
        }
        opts
    }
}
