extern crate failure;
extern crate k8s_openapi;
extern crate reqwest;

mod error;

use error::{Error, ErrorKind, Result};
use k8s_openapi::v1_10::api::core::v1 as core;
use reqwest::{header, Method};
use std::io::Read;

fn inner<T>(request: k8s_openapi::http::Request<Vec<u8>>) -> Result<T>
where
    T: k8s_openapi::Response,
{
    let (parts, body) = request.into_parts();
    let url = format!("http://localhost:8001{}", parts.uri.to_string());
    let client = reqwest::Client::new();
    let builder = match parts.method {
        Method::GET => client.get(&url),
        Method::POST => client
            .post(&url)
            .header(
                header::CONTENT_TYPE,
                header::HeaderValue::from_static("application/json"),
            )
            .body(body),
        Method::PUT => client
            .put(&url)
            .header(
                header::CONTENT_TYPE,
                header::HeaderValue::from_static("application/json"),
            )
            .body(body),
        Method::PATCH => client
            .patch(&url)
            .header(
                header::CONTENT_TYPE,
                header::HeaderValue::from_static("application/json"),
            )
            .body(body),
        Method::DELETE => client.delete(&url),
        _ => return Err(Error::from(ErrorKind::UnexpectedMethod)),
    };

    let mut response: reqwest::Response = builder.send()?;
    let mut buffer: Vec<u8> = vec![];
    response.read_to_end(&mut buffer)?;

    let status = response.status();
    let mut body = k8s_openapi::ResponseBody::new(status);
    let response = body.append_slice_and_parse(&buffer)?;

    Ok(response)
}

fn main() -> Result<()> {
    let request = core::Pod::list_core_v1_namespaced_pod(
        "kube-system",
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    )?;

    let response: core::ListCoreV1NamespacedPodResponse = inner(request)?;

    println!("{:?}", response);

    Ok(())
}
