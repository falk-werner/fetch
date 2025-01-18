use clap::Parser;
use futures_util::StreamExt;
use reqwest::{ClientBuilder, redirect::Policy};
use reqwest::{Method, Response};
use reqwest::multipart::Form;
use std::fs::File;
use std::path::PathBuf;
use std::time::Duration;
use std::process::exit;
use std::io::{Write, Read};
use sha2::{Sha256, Digest};
use md5::Md5;

/// Download an artifact from a given url and optionally verify checksum.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// URL of the artifact to fetch.
    url: String,

    /// Write to file instead of stdout.
    #[arg(short, long)]
    output: Option<String>,

    /// Specify the request method to use.
    #[arg(short='X', long)]
    request: Option<String>,

    /// Pass custom header(s) to server.
    #[arg(short='H', long)]
    header: Vec<String>,

    /// Post data.
    #[arg(short, long)]
    data: Option<String>,

    /// Specify multipart form data as name=value pair.
    #[arg(short='F', long)]
    form: Vec<String>,

    /// Allow insecure server connections.
    #[arg(short='K', long)]
    insecure: bool,

    /// Follow redirects.
    #[arg(short='L', long)]
    location: bool,

    /// Maximum number of redirects allowed.
    #[arg(long, default_value_t=5)]
    max_redirs: usize,

    /// Maximum file size to download.
    #[arg(long, default_value_t=0)]
    max_filesize: u64,

    /// Maximum time allowed for connection in seconds.
    #[arg(long, default_value_t=0)]
    connect_timeout: u64,

    /// Maximum time allowed for transfer in seconds.
    #[arg(short, long, default_value_t=0)]
    max_time: u64,

    /// SHA256 checksum of the artifact to download.
    #[arg(long)]
    sha256: Option<String>,

    /// MD5 checksum of the artifacto to download.
    #[arg(long)]
    md5: Option<String>,
}

fn get_request_method(args: & Args) -> Method {
    if let Some(request_method) = &args.request {
        let request_method = request_method.to_lowercase();
        match request_method.as_str() {
            "get" => Method::GET,
            "put" => Method::PUT,
            "post" => Method::POST,
            "delete" => Method::DELETE,
            "head" => Method::HEAD,
            "options" => Method::OPTIONS,
            "connect" => Method::CONNECT,
            "patch" => Method::PATCH,
            "trace" => Method::TRACE,
            _ => {
                eprintln!("error: invalid request method");
                exit(1);
            }
        }
    }
    else if args.data.is_some() || !args.form.is_empty() {
            Method::POST
    }
    else {
        Method::GET
    }
}

fn get_filename(filename: &Option<String>) -> PathBuf {
    if let Some(name) = filename {
        PathBuf::from(name)
    }
    else {
        let temp = tempfile::Builder::new()
            .prefix("fetch-")
            .suffix(".download")
            .tempfile().unwrap();
        PathBuf::from(temp.path())
    }
}

async fn download(response: Response,args: &Args, filename: &PathBuf) {
    let file = std::fs::File::create(filename.clone());
    if file.is_err() {
        eprint!("error: failed to create file");
        exit(1);
    }
    let mut file = file.unwrap();

    let mut md5_hasher = Md5::new();
    let mut sha256_hasher = Sha256::new();
    let mut count : u64 = 0;
    let mut stream = response.bytes_stream();
    while let Some(item) = stream.next().await {
        if item.is_err() {
            eprintln!("error: failed to read reponse data");
            std::fs::remove_file(filename).unwrap();
            exit(1);
        }

        let data = item.unwrap();
        count += data.len() as u64;
        if args.max_filesize > 0 && count > args.max_filesize {
            eprintln!("error: content length too large: expected max. {} bytes, but {} bytes received", args.max_filesize, count);
            std::fs::remove_file(filename).unwrap();
            exit(1);
        }

        if file.write_all(data.as_ref()).is_err() {
            eprintln!("error: failed to write file");
            std::fs::remove_file(filename).unwrap();
            exit(1);
        }

        sha256_hasher.update(data.as_ref());
        md5_hasher.update(data.as_ref());
    }

    // check MD5
    if let Some(expected) = &args.md5 {
        let expected = expected.to_lowercase();

        let hash = md5_hasher.finalize();
        let actual = hex::encode(hash).to_lowercase();
        if  actual != expected {
            eprintln!("error: MD5 checksum mismatch: expected {} but was {}",
                expected, actual);
            std::fs::remove_file(filename).unwrap();
            exit(1);
        }
    }

    // check SHA256
    if let Some(expected) = &args.sha256 {
        let expected = expected.to_lowercase();

        let hash = sha256_hasher.finalize();
        let actual = hex::encode(hash).to_lowercase();
        if actual != expected {
            eprintln!("error: SHA256 checksum mismatch: expected {} but was {}",
                expected, actual);
            let _ = std::fs::remove_file(filename);
            exit(1);
        }
    }


}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let request_method = get_request_method(&args);

    let mut builder = ClientBuilder::new();

    // set redirect policy
    if args.location {
        builder = builder.redirect(Policy::limited(args.max_redirs));
    }
    else {
        builder = builder.redirect(Policy::none());
    }

    // timeout
    if args.max_time > 0 {
        builder = builder.timeout(Duration::from_secs(args.max_time));
    }

    // connect timeout
    if args.connect_timeout > 0 {
        builder = builder.connect_timeout(Duration::from_secs(args.connect_timeout));
    }

    // insecure
    if args.insecure {
        builder = builder
            .danger_accept_invalid_hostnames(true)
            .danger_accept_invalid_certs(true);
    }

    let client = builder.build();
    if client.is_err() {
        eprintln!("error: failed to create http client");
        exit(1);
    }
    let client = client.unwrap();
    let mut request_builder = client.request(request_method, args.url.clone());

    // additional headers
    for x in &args.header {
        if let Some((name, value)) = x.split_once(':') {
            let name = name.trim();
            let value = value.trim();

            request_builder = request_builder.header(name, value);
        }
    }

    // data
    if let Some(ref data) = args.data {
        request_builder = request_builder.body(data.clone());
    }
    // multipart data
    else if !args.form.is_empty() {
        let mut form_data = Form::new();
        for key_value_pair in &args.form {
            if let Some((key, value)) = key_value_pair.split_once('=') {
                let key = key.trim();

                form_data = form_data.text(String::from(key), String::from(value));
            }
        }
        request_builder = request_builder.multipart(form_data);
    }

    let response = request_builder.send().await;
    if let Err(err) = response {
        eprint!("error: {}", err);
        exit(1);
    }
    let response = response.unwrap();

    let status = response.status();
    if !status.is_success() {
        eprint!("error: bad http status: {}: {}", status.as_u16(), status.as_str());
        exit(1);
    }

    if args.max_filesize > 0 {
        if let Some(content_length) = response.content_length() {
            if content_length > args.max_filesize {
                eprintln!("error: content length too large: {} bytes max. expected, but {} bytes content length", args.max_filesize, content_length);
                exit(1);
            }
        }
    }

    let filename = get_filename(&args.output);
    download(response, &args, &filename).await;

    
    if args.output.is_none() {
        let file = File::open(filename.clone());
        if file.is_err() {
            eprintln!("error: failed to open file");
            exit(1);
        }
        let mut file = file.unwrap();

        let mut buffer: [u8; 10 * 1024]= [0; 10 * 1024]; 
        loop {
            let result = file.read(&mut buffer);
            if result.is_err() {
                eprintln!("error: failed to read file");
                let _ = std::fs::remove_file(filename);
                exit(1);
            }

            let i = result.unwrap();
            if i > 0 {
                let data = &buffer[0..i];
                let _ = std::io::stdout().write_all(data.as_ref());
            }
            else {
                break;
            }
        }

        let _ = std::fs::remove_file(filename);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_filename() {
        let foo = Some(String::from("foo"));
        let actual = get_filename(&foo);
        assert_eq!(PathBuf::from("foo"), actual);

        let actual = get_filename(&None);
        assert!(std::fs::exists(actual).is_ok());
    }

    fn args_from_method(method: Option<String>, data: Option<String>) -> Args {
        Args {
            url: String::from(""),
            output: None,
            request: method,
            header: Vec::new(),
            data: data,
            form: Vec::new(),
            insecure: false,
            location: false,
            max_redirs: 0,
            max_filesize: 0,
            connect_timeout: 0,
            max_time: 0,
            sha256: None,
            md5: None,
        }
    }

    #[test]
    fn test_get_request_method() {
        let args = args_from_method(Some(String::from("get")), None);
        assert_eq!(Method::GET, get_request_method(&args));

        let args = args_from_method(Some(String::from("GET")), None);
        assert_eq!(Method::GET, get_request_method(&args));
        
        let args = args_from_method(Some(String::from("PUT")), None);
        assert_eq!(Method::PUT, get_request_method(&args));

        let args = args_from_method(Some(String::from("POST")), None);
        assert_eq!(Method::POST, get_request_method(&args));

        let args = args_from_method(Some(String::from("DELETE")), None);
        assert_eq!(Method::DELETE, get_request_method(&args));

        let args = args_from_method(Some(String::from("HEAD")), None);
        assert_eq!(Method::HEAD, get_request_method(&args));

        let args = args_from_method(Some(String::from("OPTIONS")), None);
        assert_eq!(Method::OPTIONS, get_request_method(&args));

        let args = args_from_method(Some(String::from("CONNECT")), None);
        assert_eq!(Method::CONNECT, get_request_method(&args));

        let args = args_from_method(Some(String::from("PATCH")), None);
        assert_eq!(Method::PATCH, get_request_method(&args));

        let args = args_from_method(Some(String::from("TRACE")), None);
        assert_eq!(Method::TRACE, get_request_method(&args));

        let args = args_from_method(None, Some(String::from("")));
        assert_eq!(Method::POST, get_request_method(&args));

        let args = args_from_method(None, None);
        assert_eq!(Method::GET, get_request_method(&args));
    }
}