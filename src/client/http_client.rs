use reqwest::Client;
use reqwest::header::HeaderMap;

pub fn get_http_client(token: String) -> Client {
    let mut headers = HeaderMap::with_capacity(2);
    headers.insert(
        reqwest::header::AUTHORIZATION,
        reqwest::header::HeaderValue::from_str(format!("Token {token}").as_str()).expect("REASON"),
    );
    headers.insert(
        reqwest::header::CONTENT_TYPE,
        reqwest::header::HeaderValue::from_static("text/plain; charset=utf-8"),
    );
    Client::builder()
        .default_headers(headers)
        // .min_tls_version(Version::TLS_1_3)
        // .use_rustls_tls()
        .build()
        .unwrap()
}