pub struct FizzResult {
    pub status_code: String,
    pub headers: String,
    pub body: String,
}

pub async fn get_request(
    url: &str,
    user_agent: String,
    verbose: bool,
) -> Result<FizzResult, reqwest::Error> {
    let client = reqwest::Client::builder()
        .user_agent(user_agent)
        .connection_verbose(verbose)
        .build()?;

    let res = client.get(url).send().await?;
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
        let res = get_request("http://httpbin.org/get", "rusty".to_string(), true)
            .await
            .unwrap();
        println!("{}", Colour::Red.paint(res.status_code));
        println!("{}", Colour::Green.paint(res.headers));
        println!("{}", Colour::Blue.paint(res.body));
    }

    #[tokio::test]
    #[should_panic]
    async fn test_get_header_error() {
        let res = get_request("httpjhb://httpbin.org/get", "rusty".to_string(), true)
            .await
            .unwrap();
        println!("{}", Colour::Red.paint(res.status_code));
        println!("{}", Colour::Green.paint(res.headers));
        println!("{}", Colour::Blue.paint(res.body));
    }
}
