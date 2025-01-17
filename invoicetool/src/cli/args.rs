
use std::path::PathBuf;

use clap::{arg, command, value_parser, Arg, ArgAction, Command};

#[derive(Debug)]
pub struct Args {
    pub invoice_file_path: PathBuf,
    pub auto_confirm: bool,
}

pub fn parse() -> Args {
    let matches = command!() // requires `cargo` feature
        .arg(arg!([name] "Optional name to operate on"))
        .arg(
            arg!(
                -c --config <FILE> "Sets a custom config file"
            )
            // We don't have syntax yet for optional options, so manually calling `required`
            .required(false)
            .value_parser(value_parser!(PathBuf)),
        )
        .arg(arg!(
            -d --debug ... "Turn debugging information on"
        ))
        .subcommand(
            Command::new("rename")
                .about("renames an invoice to a name based on contents")
                    .arg(arg!(-f --file <FILE> "an invoice document")
                    .required(true)
                    .value_parser(value_parser!(PathBuf)))
                    .arg(Arg::new("yes")
                    .long("yes")
                    .short('y')
                    .action(clap::ArgAction::SetTrue))
        )
        .get_matches();

    let mut temp_invoice_file_path: Option<PathBuf> = None;
    let mut temp_auto_confirm: bool = false;

    if let Some(matches) = matches.subcommand_matches("rename") {
        if let Some(invoice_file_path) = matches.get_one::<PathBuf>("file") {
            temp_invoice_file_path = Some(invoice_file_path.clone());
        }
        temp_auto_confirm = matches.get_flag("yes");
    }

    Args {
        invoice_file_path: temp_invoice_file_path.expect("unexpected missing invoice file path"),
        auto_confirm: temp_auto_confirm,
    }
}