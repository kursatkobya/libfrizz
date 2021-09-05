use std::env;
use std::io::Error;

use ansi_term::Colour;
use clap::{load_yaml, App};
use dprint_core::formatting::PrintOptions;

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
        let verbose = cmd_args.is_present("verbose");
        let target = cmd_args.value_of("target").unwrap();
        let user_agent = if cmd_args.is_present("user-agent") {
            String::from(cmd_args.value_of("user-agent").unwrap())
        } else {
            format!("frizz / {}", frizz_version)
        };
        let res = libfrizz::get_request(target, user_agent, verbose)
            .await
            .ok()
            .unwrap();
        let body = res.body;
        println!("{}", Colour::Green.paint(res.status_code));
        if cmd_args.is_present("dump-header") {
            println!("{}", Colour::Blue.paint(res.headers));
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
            println!("{}", out_prep);
        } else {
            println!("{}", Colour::White.paint(body));
        }
    }

    Ok(())
}
