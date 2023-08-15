use crate::unwrap_or_return;
use bytes::Buf;
use hyper::body;
use hyper::client::connect::Connect;
use hyper::header::CONTENT_TYPE;
use hyper::{Body, Client, StatusCode, Uri};
use std::io::Read;
use std::str::FromStr;

pub async fn send_request<C, T>(
    client: &Client<C>,
    method: &str,
    request_url: url::Url,
    data: Option<String>,
) -> Result<T, String>
where
    C: Connect + Clone + Send + Sync + 'static,
    T: serde::de::DeserializeOwned,
{
    let request_uri: Uri = FromStr::from_str(request_url.as_str()).unwrap();
    let request_builder = hyper::Request::builder()
        .method(method)
        .uri(request_uri)
        .header(CONTENT_TYPE, "application/json");
    let request = unwrap_or_return!(match data {
        Some(body) => request_builder.body(Body::from(body)),
        None => request_builder.body(Body::empty()),
    });

    let response = unwrap_or_return!(client.request(request).await);
    let status = response.status();
    let mut body_reader = unwrap_or_return!(body::aggregate(response).await).reader();

    match status {
        StatusCode::OK => Ok(unwrap_or_return!(serde_json::from_reader(body_reader))),
        _ => {
            let mut body = String::new();
            unwrap_or_return!(body_reader.read_to_string(&mut body));
            Err(format!("Error: {}. {}", status, body))
        }
    }
}
