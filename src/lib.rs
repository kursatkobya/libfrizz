mod port_details;

use ansi_term::Colour;
use futures::{lock::Mutex, stream, StreamExt};
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::{Body, Method, Request, Url};
use std::cmp::min;
use std::fs::File;
use std::{
    fs, io,
    io::{Error, Write},
    net::{IpAddr, SocketAddr},
    str::FromStr,
    sync::Arc,
    time::Duration,
    time::Instant,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, Interest},
    net::TcpStream,
};

pub struct FizzResult {
    pub status_code: String,
    pub headers: String,
    pub body: String,
}

pub struct ExecRequest {
    pub url: String,
    pub user_agent: String,
    pub verbose: bool,
    pub disable_cert_validation: bool,
    pub disable_hostname_validation: bool,
    pub post_data: String,
    pub http_method: Method,
    pub progress_bar: bool,
}

pub async fn execute_request(exec: ExecRequest) -> Result<FizzResult, reqwest::Error> {
    let client = reqwest::Client::builder()
        .user_agent(exec.user_agent)
        .danger_accept_invalid_certs(exec.disable_cert_validation)
        .danger_accept_invalid_hostnames(exec.disable_hostname_validation)
        .connection_verbose(exec.verbose)
        .build()?;

    let mut req = Request::new(exec.http_method, Url::from_str(&exec.url).unwrap());

    if !exec.post_data.is_empty() {
        let mut postdata = exec.post_data;
        if postdata.starts_with('@') {
            let file_name = postdata.split('@').last().unwrap();
            log::info!("File opening for read:{}", file_name);
            let contents = fs::read(file_name).expect("Something went wrong reading the file");
            unsafe {
                postdata = String::from_utf8_unchecked(contents);
            }
        }
        req.body_mut().replace(Body::from(postdata));
    }

    let res = client.execute(req).await.unwrap();
    let headers = res.headers().clone();
    let status_code = res.status().to_string();
    let content_length = res.content_length().unwrap_or(0);

    if content_length > 104647460 || exec.progress_bar {
        // if response content bigger then 100MB we download it with progress bar
        let total_size = content_length;
        let pb = ProgressBar::new(total_size);
        pb.set_style(ProgressStyle::default_bar()
            .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
            .progress_chars("#>-"));
        pb.set_message(format!("Executing {}", exec.url));

        let mut path = "frizz.out.file";
        if exec.url.split('/').last().unwrap().chars().count() > 1 {
            path = exec.url.split('/').last().unwrap();
        };
        // download chunks
        let mut file = File::create(path).unwrap();
        let mut downloaded: u64 = 0;
        let mut stream = res.bytes_stream();

        while let Some(item) = stream.next().await {
            let chunk = item?;
            file.write_all(&chunk).unwrap();
            let new = min(downloaded + (chunk.len() as u64), total_size);
            downloaded = new;
            pb.set_position(new);
        }

        pb.finish_with_message(format!("Downloaded {} to {}", exec.url, path));

        return Ok(FizzResult {
            status_code,
            headers: format!("Headers:\n{:#?}", headers),
            body: format!("written to ./{}", path),
        });
    }

    Ok(FizzResult {
        status_code,
        headers: format!("Headers:\n{:#?}", headers),
        body: res.text().await?,
    })
}

fn get_ports(min_port: u16, max_port: u16) -> (Box<dyn Iterator<Item = u16>>, u16) {
    if min_port == 0 && max_port == 0 {
        (
            Box::new(port_details::MOST_COMMON_PORTS.to_owned().into_iter()),
            port_details::MOST_COMMON_PORTS.len() as u16,
        )
    } else {
        (Box::new(min_port..=max_port), max_port - min_port)
    }
}

pub async fn scan(
    target: IpAddr,
    concurrency: usize,
    timeout: u64,
    min_port: u16,
    max_port: u16,
    mut out_writer: Box<dyn Write>,
) {
    let (port_box, progress_size) = get_ports(min_port, max_port);
    let ports = stream::iter(port_box);
    let output_values = Arc::new(Mutex::new(Vec::new()));
    let before = Instant::now();

    let pb = ProgressBar::new(progress_size.into());
    pb.set_style(ProgressStyle::default_bar()
        .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos:>}/{len}  ({percent}%, {eta})")
        .progress_chars("##-"));
    pb.set_message(format!("Scanning ports for {}", target));

    ports
        .for_each_concurrent(concurrency, |port| {
            let output_values = output_values.clone();
            pb.inc(1);
            async move {
                let result = scan_port(target, port, timeout).await;
                if result > 0 {
                    output_values.lock().await.push(result);
                }
            }
        })
        .await;

    pb.finish();
    for i in output_values.lock().await.iter() {
        out_writer
            .write(Colour::Blue.paint(format!("{:?}\n", i)).as_bytes())
            .ok();
    }

    println!("Elapsed time to scan ports: {:.2?}", before.elapsed());
}

async fn scan_port(target: IpAddr, port: u16, timeout: u64) -> u16 {
    let timeout = Duration::from_secs(timeout);
    let socket_address = SocketAddr::new(target, port);

    if let Ok(Ok(_)) = tokio::time::timeout(timeout, TcpStream::connect(&socket_address)).await {
        return port;
    }
    0
}

pub async fn open_socket_target(target: &str) -> Result<(), Error> {
    log::info!("Socket connection");

    let t_url = Url::parse(target).unwrap();
    let addrs = t_url.socket_addrs(|| None).unwrap();
    let mut stream = TcpStream::connect(&*addrs).await?;

    loop {
        let ready = stream
            .ready(Interest::READABLE | Interest::WRITABLE)
            .await?;
        let mut data = vec![];
        if ready.is_writable() {
            let prompt = format!("{}{:?}{}", "Connected ", stream.peer_addr(), ">");
            print!("{}", Colour::Green.paint(prompt));
            io::stdout().flush().ok();

            let mut input = String::new();
            io::stdin().read_line(&mut input).ok();
            if input.trim().eq_ignore_ascii_case("exit") {
                return Ok(());
            }
            stream.write_all(input.as_bytes()).await?;
            stream.read_to_end(&mut data).await?;
            println!("Response:{:?}", String::from_utf8(data));
        }
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use ansi_term::Colour;

    use super::*;

    #[tokio::test]
    async fn test_get_header() {
        let res = execute_request(ExecRequest {
            url: "http://httpbin.org/get".to_string(),
            user_agent: "rusty".to_string(),
            verbose: true,
            disable_cert_validation: true,
            disable_hostname_validation: true,
            post_data: "".to_string(),
            http_method: Method::GET,
            progress_bar: true,
        })
        .await
        .unwrap();
        println!("{}", Colour::Red.paint(res.status_code));
        println!("{}", Colour::Green.paint(res.headers));
        println!("{}", Colour::Blue.paint(res.body));
    }

    #[tokio::test]
    #[should_panic]
    async fn test_get_header_error() {
        let res = execute_request(ExecRequest {
            url: "htasxatp://httpbin.org/get".to_string(),
            user_agent: "rusty".to_string(),
            verbose: true,
            disable_cert_validation: true,
            disable_hostname_validation: true,
            post_data: "".to_string(),
            http_method: Method::GET,
            progress_bar: true,
        })
        .await
        .unwrap();
        println!("{}", Colour::Red.paint(res.status_code));
        println!("{}", Colour::Green.paint(res.headers));
        println!("{}", Colour::Blue.paint(res.body));
    }
}
