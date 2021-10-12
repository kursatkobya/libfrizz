use std::str::FromStr;

use reqwest::{Body, Method, Request, Url};
use tokio::{net::TcpStream};
use std::{net::{IpAddr, SocketAddr},
          time::Duration,};
use futures::{stream, StreamExt};

pub struct FizzResult {
    pub status_code: String,
    pub headers: String,
    pub body: String,
}

pub async fn execute_request(
    url: &str,
    user_agent: String,
    _verbose: bool,
    _disable_cert_validation: bool,
    _disable_hostname_validation: bool,
    post_data: Option<&str>,
    http_method: Method,
) -> Result<FizzResult, reqwest::Error> {
    let client = reqwest::Client::builder()
        .user_agent(user_agent)
        .danger_accept_invalid_certs(_disable_cert_validation)
        .danger_accept_invalid_hostnames(_disable_hostname_validation)
        .connection_verbose(_verbose)
        .build()?;

    let mut req = Request::new(http_method, Url::from_str(url).unwrap());

    if post_data.is_some() {
        req.body_mut()
            .replace(Body::from(String::from(post_data.unwrap())));
    }

    let res = client.execute(req).await?;

    Ok(FizzResult {
        status_code: res.status().to_string(),
        headers: format!("Headers:\n{:#?}", res.headers()),
        body: res.text().await?,
    })
}

pub async fn scan(target: IpAddr, concurrency: usize, timeout: u64, min_port:u16, max_port:u16) {
    let ports = stream::iter(min_port..=max_port);

    ports
        .for_each_concurrent(concurrency, |port| scan_port(target, port, timeout))
        .await;
}

async fn scan_port(target: IpAddr, port: u16, timeout: u64) {
    let timeout = Duration::from_secs(timeout);
    let socket_address = SocketAddr::new(target, port);

    if tokio::time::timeout(timeout, TcpStream::connect(&socket_address))
        .await
        .is_ok()
    {
        println!("{}", port);
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use ansi_term::Colour;

    use super::*;

    #[tokio::test]
    async fn test_get_header() {
        let res = execute_request(
            "http://httpbin.org/get",
            "rusty".to_string(),
            true,
            true,
            true,
            Option::from(""),
            Method::GET,
        )
        .await
        .unwrap();
        println!("{}", Colour::Red.paint(res.status_code));
        println!("{}", Colour::Green.paint(res.headers));
        println!("{}", Colour::Blue.paint(res.body));
    }

    #[tokio::test]
    #[should_panic]
    async fn test_get_header_error() {
        let res = execute_request(
            "httpjhb://httpbin.org/get",
            "rusty".to_string(),
            true,
            true,
            true,
            Option::from(""),
            Method::GET,
        )
        .await
        .unwrap();
        println!("{}", Colour::Red.paint(res.status_code));
        println!("{}", Colour::Green.paint(res.headers));
        println!("{}", Colour::Blue.paint(res.body));
    }
}
