use std::io::{Error, Write};
use std::{env, io,
          net::{IpAddr, SocketAddr, ToSocketAddrs},
          time::Duration,};
use futures::{stream, StreamExt};
use ansi_term::Colour;
use clap::{load_yaml, App, ArgMatches};
use dprint_core::formatting::PrintOptions;
use reqwest::{Method, Url};
use std::fs::File;
use std::path::Path;

use tokio::io::{AsyncReadExt, Interest};
use tokio::{io::AsyncWriteExt, net::TcpStream};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let frizz_version = env!("CARGO_PKG_VERSION");
    let args: Vec<String> = env::args().collect();
    let yaml = load_yaml!("cli.yaml");
    let cmd_args = App::from(yaml).get_matches();

    if args.iter().len() == 1 {
        App::from(yaml).print_help().ok();
        println!("\nERROR:Please provide the parameters");
        return Ok(());
    }

    if cmd_args.is_present("target") {

        let target = cmd_args.value_of("target").unwrap();

        let insecure = cmd_args.is_present("insecure");
        let verbose = cmd_args.is_present("verbose");
        if verbose {
            simple_logger::init().unwrap();
        } else {
            simple_logger::init_with_level(log::Level::Info).unwrap();
        }
        let user_agent = if cmd_args.is_present("user-agent") {
            String::from(cmd_args.value_of("user-agent").unwrap())
        } else {
            format!("frizz / {}", frizz_version)
        };

        let method = match cmd_args.value_of("request") {
            Some(x) => Method::from_bytes(x.as_bytes()).unwrap(),
            _ => Method::GET,
        };

        if target.starts_with("http") {
            execute_http(
                cmd_args.clone(),
                insecure,
                verbose,
                target,
                user_agent,
                method,
            )
            .await
        } else {
            if cmd_args.is_present("scan") {
                let socket_addresses: Vec<SocketAddr> = format!("{}:0", target).to_socket_addrs()?.collect();
                if socket_addresses.is_empty() {
                    println!("Socket_addresses list is empty");
                    return Ok(());
                }
                println!("ip addr {}", socket_addresses[0].ip());
                scan(socket_addresses[0].ip(), 1000, 1).await;
                return Ok(());
            }

            // assume telnet or any socket protocol
            open_socket_target(target).await.ok();
        }
    }

    Ok(())
}

async fn open_socket_target(target: &str) -> Result<(), Error> {
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

async fn execute_http(
    cmd_args: ArgMatches<'_>,
    insecure: bool,
    verbose: bool,
    target: &str,
    user_agent: String,
    method: Method,
) {
    let res = libfrizz::execute_request(
        target,
        user_agent,
        verbose,
        insecure,
        insecure, // TODO: we can accept this from another parameter, for now insecure covers.
        cmd_args.value_of("data"),
        method,
    )
    .await
    .ok()
    .unwrap();
    let body = res.body;

    let mut out_writer = match cmd_args.value_of("output") {
        Some(x) => {
            let path = Path::new(x);
            Box::new(File::create(&path).unwrap()) as Box<dyn Write>
        }
        None => Box::new(io::stdout()) as Box<dyn Write>,
    };

    if cmd_args.is_present("fail") && !res.status_code.contains("200") {
        out_writer = Box::new(io::sink()) as Box<dyn Write>;
    }

    out_writer
        .write(format!("{}", Colour::Green.paint(res.status_code)).as_bytes())
        .ok();
    if cmd_args.is_present("dump-header") {
        out_writer
            .write(format!("{}", Colour::Blue.paint(res.headers)).as_bytes())
            .ok();
    }

    if cmd_args.is_present("pretty") {
        let opts = PrintOptions {
            indent_width: 4,
            max_width: 10,
            use_tabs: false,
            new_line_text: "\n",
        };
        let items = dprint_core::formatting::parser_helpers::parse_string(body.as_str());
        let out_prep = Colour::White.paint(dprint_core::formatting::format(|| items, opts));
        out_writer.write(out_prep.as_bytes()).ok();
    } else {
        out_writer
            .write(format!("{}", Colour::White.paint(body)).as_bytes())
            .ok();
    }
}


async fn scan(target: IpAddr, concurrency: usize, timeout: u64) {
    let ports = stream::iter(1..=u16::MAX);

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