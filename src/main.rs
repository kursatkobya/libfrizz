use std::convert::TryFrom;
use std::env;
use std::io::Error;

use ansi_term::Colour;
use clap::{load_yaml, App};
use dprint_core::formatting::{PrintItems, PrintOptions};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let yaml = load_yaml!("cli.yaml");
    let cmd_args = App::from(yaml).get_matches();

   if args.iter().len() == 1 {
        App::from(yaml).print_help().ok();
        println!("\nERROR:Please provide the parameters");
        return Ok(());
    }

    if cmd_args.is_present("target") {
        let res = libfrizz::get_header(cmd_args.value_of("target").unwrap())
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
            let items = PrintItems::try_from(body).unwrap();
            let out_prep = Colour::White.paint(dprint_core::formatting::format(|| items, opts));
            println!("{}", out_prep);
        } else {
            println!("{}", Colour::White.paint(body));
        }
    }

    Ok(())
}
