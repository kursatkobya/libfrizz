use std::convert::TryFrom;
use std::env;
use std::io::{Error, ErrorKind};

use ansi_term::Colour;
use clap::{App, load_yaml};
use dprint_core::formatting::{PrintItems, PrintOptions};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let yaml = load_yaml!("cli.yaml");
    let cmd_args = App::from(yaml).get_matches();

    if args.len() == 1 {
        App::from(yaml).print_help();
        return Err(Error::new(ErrorKind::Other, "\n\nPlease provide the parameters!"));
    }

    if cmd_args.is_present("target") {
        let res = libfrizz::get_header(cmd_args.value_of("target").unwrap()).await.ok().unwrap();
        let body = res.body;
        println!("{}", Colour::Green.paint(res.status_code));
        if cmd_args.is_present("dump-header") {
            println!("{}", Colour::Blue.paint(res.headers));
        }

        if cmd_args.is_present("pretty"){
            println!("{}", Colour::White.paint(
                dprint_core::formatting::format(|| {
                    {
                        PrintItems::try_from(body).unwrap()
                    }
                }, PrintOptions {
                    indent_width: 4,
                    max_width: 10,
                    use_tabs: false,
                    new_line_text: "\n",
                })
            ));
        } else {
            println!("{}", Colour::White.paint(body));
        }
    }

    Ok(())
}
