use std::{env, io,
          io::{Error, Write},
          net::{SocketAddr, ToSocketAddrs},
          fs::File,path::Path,
          cmp
};
use ansi_term::Colour;
use clap::{load_yaml, App, ArgMatches};
use dprint_core::formatting::PrintOptions;
use reqwest::Method;


#[tokio::main]
async fn main() -> Result<(), Error> {
    let frizz_version = env!("CARGO_PKG_VERSION");
    let yaml = load_yaml!("cli.yaml");
    let cmd_args = App::from(yaml).get_matches();

    if !cmd_args.is_present("target")  {
        println!("\nERROR:Please provide the parameters\n");
        App::from(yaml).print_help().ok();
        return Ok(());
    }
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
    } else if cmd_args.is_present("scan") {
            let socket_addresses: Vec<SocketAddr> = format!("{}:0", target).to_socket_addrs()?.collect();
            if socket_addresses.is_empty() {
                println!("Socket_addresses list is empty");
                return Ok(());
            }

            let timeout= cmd_args
                              .value_of("timeout")
                              .unwrap()
                              .parse::<u64>()
                              .unwrap_or(1);
            let concurrency = cmd_args
                                    .value_of("concurrency")
                                    .unwrap()
                                    .parse::<usize>()
                                    .unwrap_or(1024);

            let mut port1: u16 = 80; let mut port2: u16 = 1024;
            if cmd_args.is_present("ports") {
                let port_range:Vec<&str> = cmd_args
                    .values_of("ports")
                    .unwrap()
                    .collect();
                if !port_range.is_empty() {
                    port1 = port_range[0].parse()
                        .expect("Unexpected port entry: Enter a valid port number");
                    port2 = port_range[1].parse()
                        .expect("Unexpected port entry: Enter a valid port number");
                }
            }

            libfrizz::scan(socket_addresses[0].ip(),
                           concurrency,
                           timeout,
                           cmp::min(port1,port2),
                           cmp::max(port1,port2)).await;
            return Ok(());
    } else {
        // assume telnet or any socket protocol
        libfrizz::open_socket_target(target).await.ok();
    }

    Ok(())
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