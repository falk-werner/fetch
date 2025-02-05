use axum::{
    extract::Multipart,
    extract::Request,
    http::header::USER_AGENT,
    routing::get,
    routing::post,
    routing::put,
    routing::patch,
    routing::delete,
    response::Response,
    Router,
};

use axum_server::tls_rustls::RustlsConfig;
use std::{thread, time::Duration, net::SocketAddr};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(welcome))
        .route("/slow_answer", get(slow_answer))
        .route("/echo_post", post(echo_data))
        .route("/echo_put", put(echo_data))
        .route("/echo_patch", patch(echo_data))
        .route("/echo_form", post(echo_form))
        .route("/delete", delete(do_delete))
        .route("/user_agent", get(get_user_agent))
        .route("/error", get(get_error))
        ;

    // configure certificate and private key used by https
    let key = include_bytes!("key.pem").to_vec();
    let cert = include_bytes!("cert.pem").to_vec();
    let config = RustlsConfig::from_pem(cert, key)
        .await
        .unwrap();

    // run https server
    let addr = SocketAddr::from(([127, 0, 0, 1], 9000));
    axum_server::bind_rustls(addr, config)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn welcome() -> &'static str {
    "Welcome!"
}

async fn slow_answer() -> &'static str {
    thread::sleep(Duration::from_secs(30));
    "42"
}

async fn echo_data(body:String) -> String {
    body
}

async fn do_delete() -> &'static str {
    "Removed"
}

async fn echo_form(mut multipart: Multipart) -> String {
    let mut result = String::new();

    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = String::from(std::str::from_utf8(&field.bytes().await.unwrap()).unwrap());

        result.push_str(&format!("{} = {};", name, data));
    }

    result
}

async fn get_user_agent(request: Request) -> String {
    let user_agent_header = request.headers().get(USER_AGENT);
    let user_agent = user_agent_header.and_then(|value| value.to_str().ok());

    if let Some(user_agent) = user_agent {
        String::from(user_agent)
    }
    else {
        String::from("unknonwn")
    }
}

async fn get_error() -> Response {
    Response::builder()
        .status(500)
        .body("Something went wrong.".into())
        .unwrap()
}