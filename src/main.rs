use clap::Parser;
use log::{warn, error};
use futures_util::StreamExt;
use logger::init_logger;
use reqwest::tls::Version;
use reqwest::{ClientBuilder, redirect::Policy};
use reqwest::{header, Certificate, Method, Proxy, Response};
use reqwest::multipart::Form;
use std::fs::File;
use std::path::PathBuf;
use std::time::Duration;
use std::process::{exit, ExitCode};
use std::io::{Write, Read};
use sha2::{Sha256, Digest};
use md5::Md5;
use tokio::fs::File as TokioFile;

mod logger;
mod args;

use crate::args::Args;

struct Protocols {
    http: bool,
    https: bool,
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
                error!("invalid request method");
                exit(1);
            }
        }
    }
    else if args.data.is_some() || args.data_raw.is_some() || !args.form.is_empty() {
            Method::POST
    }
    else {
        Method::GET
    }
}

fn get_protocols(protocols: &String) -> Protocols
{
    let mut result = Protocols {http: true, https: true };
    for part in protocols.split(',') {
        if let Some(modifier) = part.trim().chars().next() {
            match modifier {
                '=' => {
                    let protocol = &part[1..];
                    match protocol {
                        "all" => { result = Protocols{http: true, https: true}; },
                        "http" => { result = Protocols{http: true, https: false}; },
                        "https" => { result = Protocols{http: false, https: true}; },
                        _ => {
                            warn!("unrecognized protocol \'{}''", protocol);
                        }
                    }
                },
                '+' => {
                    let protocol = &part[1..];
                    match protocol {
                        "all" => { result = Protocols{http: true, https: true}; },
                        "http" => { result.http = true; },
                        "https" => { result.https = true; },
                        _ => {
                            warn!("unrecognized protocol \'{}''", protocol);
                        }
                    };                
                },
                '-' => {
                    let protocol = &part[1..];
                    match protocol {
                        "all" => { result = Protocols{http: false, https: false}; },
                        "http" => { result.http = false; },
                        "https" => { result.https = false; },
                        _ => {
                            warn!("unrecognized protocol \'{}''", protocol);
                        }
                    };                
                },
                _ => {
                    warn!("unrecognized protocol \'{}''", part);
                }
            }
        }
    }
    result
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
        error!("failed to create file");
        exit(1);
    }
    let mut file = file.unwrap();

    let mut md5_hasher = Md5::new();
    let mut sha256_hasher = Sha256::new();
    let mut count : u64 = 0;
    let mut stream = response.bytes_stream();
    while let Some(item) = stream.next().await {
        if item.is_err() {
            error!("failed to read reponse data");
            std::fs::remove_file(filename).unwrap();
            exit(1);
        }

        let data = item.unwrap();
        count += data.len() as u64;
        if args.max_filesize > 0 && count > args.max_filesize {
            error!("content length too large: expected max. {} bytes, but {} bytes received", args.max_filesize, count);
            std::fs::remove_file(filename).unwrap();
            exit(1);
        }

        if file.write_all(data.as_ref()).is_err() {
            error!("failed to write file");
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
            error!("MD5 checksum mismatch: expected {} but was {}",
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
            error!("SHA256 checksum mismatch: expected {} but was {}",
                expected, actual);
            let _ = std::fs::remove_file(filename);
            exit(1);
        }
    }


}

#[tokio::main]
async fn main() -> ExitCode {
    let mut exit_code = ExitCode::SUCCESS;
    let args = Args::parse();
    init_logger(&args);

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

    // tls
    if args.tlsv1 || args.tlsv1_0 {
        builder = builder.min_tls_version(Version::TLS_1_0);
    }
    if args.tlsv1_1 {
        builder = builder.min_tls_version(Version::TLS_1_1);
    }
    if args.tlsv1_2 {
        builder = builder.min_tls_version(Version::TLS_1_2);
    }
    if args.tlsv1_3 {
        // TLS 1.3 requires rustls on some machines
        // otherwise a build error occurs
        builder = builder.use_rustls_tls()
            .min_tls_version(Version::TLS_1_3);
    }

    // protocols
    // Note that we only parse protocols to determine
    // if http_only can be enabled. The underlying
    // library does not allow to disbale https.
    let protocols = get_protocols(&args.proto);
    if !protocols.http && protocols.https {
        builder = builder.https_only(true);
    }

    // proxy
    if let Some(ref proxy) = args.proxy {
        builder = builder.proxy(Proxy::http(proxy).unwrap());
    }

    // CA certificate
    if let Some(ref cacert) = args.cacert {
        let file = File::open(cacert);
        if file.is_err() {
            eprintln!("failed to open CA certificate file");
            return ExitCode::FAILURE;
        }

        let mut file = file.unwrap();
        let mut data : Vec<u8> = vec!();
        if file.read_to_end(&mut data).is_err() {
            eprintln!("failed to read CA certifcate file");
            return ExitCode::FAILURE;
        }

        if cacert.ends_with(".der") {
            let cert = Certificate::from_der(data.as_ref());
            if cert.is_err() {
                eprintln!("failed to load DER certificate");
                return ExitCode::FAILURE;
            }
            let cert = cert.unwrap();
            builder = builder.add_root_certificate(cert);
        }
        else {
            let certs = Certificate::from_pem_bundle(data.as_ref());
            if certs.is_err() {
                eprintln!("failed to load PEM certificate bundle");
                return ExitCode::FAILURE;
            }
            let certs = certs.unwrap();
            for cert in certs {
                builder = builder.add_root_certificate(cert);
            }
        }
    }


    let client = builder.build();
    if client.is_err() {
        error!("failed to create http client");
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

    // user agent
    if let Some(ref user_agent) = args.user_agent {
        request_builder = request_builder.header(header::USER_AGENT, user_agent);
    }

    // data
    if let Some(ref data) = args.data {
        if Some('@') == data.chars().next() {
            let filename = &data[1..];
            let file = TokioFile::open(filename).await;
            if let Ok(file) = file {
                request_builder = request_builder.body(file);
            }
            else {
                warn!("failed to open file, this results in an empty request body");
            }
        } else {
            request_builder = request_builder.body(data.clone());
        }
    }
    else if let Some(ref data) = args.data_raw {
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
        error!("{}", err);
        exit(1);
    }
    let response = response.unwrap();

    // print response headers
    if args.include {
        println!("{:?} {}", response.version(), response.status());
        for (header, value) in response.headers().into_iter() {
            println!("{}: {}", header, value.to_str().unwrap());
        }
        println!();
    }

    let status = response.status();
    if !status.is_success() {
        if args.fail {
            error!("bad http status: {}", status.as_u16());
            exit(1);
        }
        if args.fail_with_body {
            error!("bad http status: {}", status.as_u16());
            exit_code = ExitCode::FAILURE;
        }
    
    }

    if args.max_filesize > 0 {
        if let Some(content_length) = response.content_length() {
            if content_length > args.max_filesize {
                error!("content length too large: {} bytes max. expected, but {} bytes content length", args.max_filesize, content_length);
                exit(1);
            }
        }
    }

    let filename = get_filename(&args.output);
    download(response, &args, &filename).await;

    
    if args.output.is_none() {
        let file = File::open(filename.clone());
        if file.is_err() {
            error!("failed to open file");
            exit(1);
        }
        let mut file = file.unwrap();

        let mut buffer: [u8; 10 * 1024]= [0; 10 * 1024]; 
        loop {
            let result = file.read(&mut buffer);
            if result.is_err() {
                error!("failed to read file");
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

    exit_code
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
            user_agent: None,
            data: data,
            data_raw: None,
            form: Vec::new(),
            insecure: false,
            location: false,
            max_redirs: 0,
            max_filesize: 0,
            connect_timeout: 0,
            max_time: 0,
            tlsv1: false,
            tlsv1_0: false,
            tlsv1_1: false,
            tlsv1_2: false,
            tlsv1_3: false,
            proto: String::from(""),
            silent: false,
            show_error: false,
            verbose: false,
            include: false,
            fail: false,
            fail_with_body: false,
            proxy: None,
            cacert: None,
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

    #[test]
    fn test_get_protocols() {
        let protocols = get_protocols(&String::from(""));
        assert!(protocols.http);
        assert!(protocols.https);

        let protocols = get_protocols(&String::from("all"));
        assert!(protocols.http);
        assert!(protocols.https);

        let protocols = get_protocols(&String::from("=http"));
        assert!(protocols.http);
        assert!(!protocols.https);

        let protocols = get_protocols(&String::from("=https"));
        assert!(!protocols.http);
        assert!(protocols.https);

        let protocols = get_protocols(&String::from("http,-https"));
        assert!(protocols.http);
        assert!(!protocols.https);

        let protocols = get_protocols(&String::from("-http,https"));
        assert!(!protocols.http);
        assert!(protocols.https);

        let protocols = get_protocols(&String::from("-all,+http"));
        assert!(protocols.http);
        assert!(!protocols.https);

    }
}