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

    /// Post data.
    #[arg(short, long)]
    pub data: Option<String>,

    /// Specify multipart form data as name=value pair.
    #[arg(short='F', long)]
    pub form: Vec<String>,

    /// Allow insecure server connections.
    #[arg(short='K', long)]
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

    /// SHA256 checksum of the artifact to download.
    #[arg(long)]
    pub sha256: Option<String>,

    /// MD5 checksum of the artifacto to download.
    #[arg(long)]
    pub md5: Option<String>,
}
