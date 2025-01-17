
use std::path::PathBuf;

use clap::{arg, command, value_parser, Arg, ArgAction, Command};

pub struct Args<'a> {
    pub invoice_file_path: &'a PathBuf,
}

pub fn parse() {
    
    println!("hi");
    
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

    if let Some(matches) = matches.subcommand_matches("rename") {
        if let Some(invoice_file_path) = matches.get_one::<PathBuf>("file") {
            println!("invoice file: {}", invoice_file_path.display());
        }
        if let Some(rename_auto_confirm) = matches.get_one::<bool>("yes") {
            println!("invoice rename auto confirm: {}", rename_auto_confirm);
        }
    }

    


        //Args {
        //    invoice_file_path: matches.get_one::<PathBuf>("config").unwrap(),
       // }
    
}