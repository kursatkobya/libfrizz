#![allow(dead_code, unused_imports)]
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::Mutex;

use chrono::{DateTime, Local};
use regex::Regex;
use reqwest::Method;
use serde_json::Value;

use crate::{execute_request, ExecRequest, TransportLayerProtocol};

pub fn get_transport_layer_protocol_name(t: TransportLayerProtocol) -> &'static str {
    match t {
        TransportLayerProtocol::None => r#"none"#,
        TransportLayerProtocol::Sctp => "sctp",
        TransportLayerProtocol::Tcp => "tcp",
        TransportLayerProtocol::Udp => "udp",
    }
}

pub fn get_most_common_ports_with_name(t: TransportLayerProtocol) -> HashMap<u16, String> {
    let ports_str = fs::read_to_string(get_service_file()).unwrap();
    let re = Regex::new(format!(r"(\w+)\t(\d+)/{}", get_transport_layer_protocol_name(t)).as_str())
        .unwrap();
    log::debug!("nmap file lines count:{}", ports_str.lines().count());
    let mut most_common_ports: HashMap<u16, String> = Default::default();
    for cap in re.captures_iter(ports_str.as_str()) {
        log::debug!("0: {} 1: {} 2:{}", &cap[0], &cap[1], &cap[2]);
        most_common_ports.insert(cap[2].parse::<u16>().unwrap(), cap[1].to_string());
    }

    most_common_ports
}

pub fn get_most_common_ports(t: TransportLayerProtocol) -> Vec<u16> {
    let ports_str = fs::read_to_string(get_service_file()).unwrap();
    let re = Regex::new(format!(r"(\w+)\t(\d+)/{}", get_transport_layer_protocol_name(t)).as_str())
        .unwrap();
    log::debug!("nmap file lines count:{}", ports_str.lines().count());
    let mut most_common_ports: Vec<u16> = Vec::new();
    for cap in re.captures_iter(ports_str.as_str()) {
        log::debug!("0: {} 1: {} 2:{}", &cap[0], &cap[1], &cap[2]);
        most_common_ports.push(cap[2].parse::<u16>().unwrap());
    }

    most_common_ports
}

pub fn get_details() -> Mutex<HashMap<u16, String>> {
    let udp_vector = get_most_common_ports_with_name(TransportLayerProtocol::Udp);
    let tcp_vector = get_most_common_ports_with_name(TransportLayerProtocol::Tcp);
    let sctp_vector = get_most_common_ports_with_name(TransportLayerProtocol::Sctp);
    let mut m: HashMap<u16, String> = Default::default();

    for p in udp_vector.iter() {
        m.insert(*p.0, format!("{}       udp", p.1));
        log::debug!("k:{} v:{}", p.0, p.1);
    }
    for p in tcp_vector.iter() {
        m.insert(*p.0, format!("{}       tcp", p.1));
        log::debug!("k:{} v:{}", p.0, p.1);
    }
    for p in sctp_vector.iter() {
        m.insert(*p.0, format!("{}       sctp", p.1));
        log::debug!("k:{} v:{}", p.0, p.1);
    }

    Mutex::new(m)
}

pub fn get_service_file() -> String {
    let s = String::from(home::home_dir().unwrap().as_path().to_str().unwrap());
    format!("{}/.frizz/nmap-services", &s)
}

pub async fn init() {
    // STEP 1... download the file
    let s = String::from(home::home_dir().unwrap().as_path().to_str().unwrap());
    let home = format!("{}/.frizz/", &s);
    let service_file = format!("{}/.frizz/nmap-services", &s);
    if !Path::exists(service_file.as_ref()) {
        // download the file it does not exist
        download_overwrite(home, &service_file).await;
    } else {
        static APP_USER_AGENT: &str =
            concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

        let client = reqwest::Client::builder()
            .user_agent(APP_USER_AGENT)
            .build()
            .unwrap();
        let last_modified_sys = fs::metadata(&service_file).unwrap().modified().unwrap();
        let last_modified = &DateTime::<Local>::from(last_modified_sys);
        // check https://github.com/nmap/nmap/blob/master/nmap-services is newer
        let query = "[0].\"commit\".\"committer\".\"date\"";

        let response = client.get(
            "https://api.github.com/repos/nmap/nmap/commits?path=nmap-services&page=1&per_page=1",
        )
        .send()
        .await
        .unwrap()
        .text()
        .await;
        let resp = response.unwrap();
        let v: Value = serde_json::from_str(resp.as_str()).unwrap();
        let r = jql::walker(&v, Some(query));
        let file_date_in_git = DateTime::parse_from_rfc3339(r.unwrap().as_str().unwrap()).unwrap();
        log::debug!("file last modified in git:{:?}", file_date_in_git);

        if file_date_in_git.gt(last_modified) {
            // check if git has newer file
            log::info!(
                "Git has newer file, download and overwrite local file.git:{}",
                file_date_in_git
            );
            download_overwrite(home, &service_file).await;
        }
    }
}

async fn download_overwrite(home: String, service_file: &str) {
    execute_request(ExecRequest {
        url: "https://raw.githubusercontent.com/nmap/nmap/master/nmap-services".to_string(),
        user_agent: "frizz".to_string(),
        verbose: false,
        disable_cert_validation: true,
        disable_hostname_validation: true,
        post_data: "".to_string(),
        http_method: Method::GET,
        progress_bar: true,
    })
    .await
    .unwrap();
    fs::create_dir(home)
        .and_then(|_| {
            fs::copy("./nmap-services", &service_file)
                .and_then(|_| fs::remove_file("nmap-services"))
        })
        .ok();
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        get_details();
        get_most_common_ports_with_name(TransportLayerProtocol::Sctp);
    }
    #[tokio::test]
    async fn async_test() {
        init().await;
    }
}
