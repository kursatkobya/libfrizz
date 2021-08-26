pub struct FizzResult {
    pub status_code: String,
    pub headers: String,
    pub body: String,
}

pub async fn get_header(url: &str) -> Result<FizzResult, reqwest::Error> {
    let res = reqwest::get(url).await?;
    Ok(FizzResult {
        status_code: res.status().to_string(),
        headers: format!("Headers:\n{:#?}", res.headers()),
        body: res.text().await?,
    })
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;
    use ansi_term::Colour;

    #[tokio::test]
    async fn test_get_header() {
        let res = get_header("http://httpbin.org/get").await.unwrap();
        println!("{}", Colour::Red.paint(res.status_code));
        println!("{}", Colour::Green.paint(res.headers));
        println!("{}", Colour::Blue.paint(res.body));
    }
    #[tokio::test]
    #[should_panic]
    async fn test_get_header_error() {
        let res = get_header("httpjhb://httpbin.org/get").await.unwrap();
        println!("{}", Colour::Red.paint(res.status_code));
        println!("{}", Colour::Green.paint(res.headers));
        println!("{}", Colour::Blue.paint(res.body));
    }
}
