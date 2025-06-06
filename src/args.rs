use clap::Parser;

/// Download an artifact from a given url and optionally verify checksum.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// URL of the artifact to fetch.
    pub url: String,

    /// Write to file instead of stdout.
    #[arg(short, long)]
    pub output: Option<String>,

    /// Specify the request method to use.
    #[arg(short='X', long)]
    pub request: Option<String>,

    /// Pass custom header(s) to server.
    #[arg(short='H', long)]
    pub header: Vec<String>,

    /// Send User-Agent to server.
    #[arg(short='A', long="user-agent")]
    pub user_agent: Option<String>,

    /// Post data.
    #[arg(short, long)]
    pub data: Option<String>,

    /// Post data, '@' allowed 
    #[arg(long="data-raw")]
    pub data_raw: Option<String>,

    /// Specify multipart form data as name=value pair.
    #[arg(short='F', long)]
    pub form: Vec<String>,

    /// Allow insecure server connections.
    #[arg(short='k', long)]
    pub insecure: bool,

    /// Follow redirects.
    #[arg(short='L', long)]
    pub location: bool,

    /// Maximum number of redirects allowed.
    #[arg(long, default_value_t=5)]
    pub max_redirs: usize,

    /// Maximum file size to download.
    #[arg(long, default_value_t=0)]
    pub max_filesize: u64,

    /// Maximum time allowed for connection in seconds.
    #[arg(long, default_value_t=0)]
    pub connect_timeout: u64,

    /// Maximum time allowed for transfer in seconds.
    #[arg(short, long, default_value_t=0)]
    pub max_time: u64,

    /// Use TLSv1.0 or later
    #[arg(short='1', long)]
    pub tlsv1 : bool,

    /// Use TLSv1.0 or later
    #[arg(long="tlsv1.0")]
    pub tlsv1_0: bool,

    /// Use TLSv1.1 or later
    #[arg(long="tlsv1.1")]
    pub tlsv1_1: bool,

    /// Use TLSv1.2 or later
    #[arg(long="tlsv1.2")]
    pub tlsv1_2: bool,

    /// Use TLSv1.3 or later
    #[arg(long="tlsv1.3")]
    pub tlsv1_3: bool,

    /// Enable or disable Protocols
    #[arg(long, allow_hyphen_values=true, default_value="")]
    pub proto: String,

    /// Silent Mode
    #[arg(short, long)]
    pub silent: bool,

    /// Show error even when -s is use
    #[arg(short='S', long="show-error")]
    pub show_error: bool,

    /// Make the operation more talkative
    #[arg(short, long)]
    pub verbose: bool,

    /// Include HTTP response headers in the output.
    #[arg(short, long)]
    pub include: bool,

    /// Fail silently (no output at all) on HTTP errors
    #[arg(short, long)]
    pub fail: bool,

    /// Fail on HTTP errors but save the body
    #[arg(long="fail-with-body")]
    pub fail_with_body: bool,

    /// [protocol://]host[:port] Use this proxy
    #[arg(short='x', long)]
    pub proxy: Option<String>,

    /// CA certificate to verify peer against
    #[arg(long)]
    pub cacert: Option<String>,

    /// Use this CRL list
    #[arg(long)]
    pub crlfile: Option<String>,

    /// SHA256 checksum of the artifact to download.
    #[arg(long)]
    pub sha256: Option<String>,

    /// MD5 checksum of the artifacto to download.
    #[arg(long)]
    pub md5: Option<String>,
}
