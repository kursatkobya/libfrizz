use std::str::FromStr;

use reqwest::{Body, Method, Request, Url};

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
    body: Option<&str>,
    method: Method,
) -> Result<FizzResult, reqwest::Error> {
    let client = reqwest::Client::builder()
        .user_agent(user_agent)
        .danger_accept_invalid_certs(_disable_cert_validation)
        .danger_accept_invalid_hostnames(_disable_hostname_validation)
        .connection_verbose(_verbose)
        .build()?;

    let mut req = Request::new(method, Url::from_str(url).unwrap());

    if body.is_some() {
        req.body_mut()
            .replace(Body::from(String::from(body.unwrap())));
    }

    let res = client.execute(req).await?;

    Ok(FizzResult {
        status_code: res.status().to_string(),
        headers: format!("Headers:\n{:#?}", res.headers()),
        body: res.text().await?,
    })
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
