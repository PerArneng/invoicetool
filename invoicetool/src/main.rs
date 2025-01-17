mod cli;

use invoicelib::add;

fn main() {
    let args = cli::args::parse();
    
    if !args.invoice_file_path.exists() {
        //FIXME: log using proper logging library
        eprintln!("Error: Invoice file '{}' does not exist", args.invoice_file_path.display());
        std::process::exit(1);
    }

    println!("args: {:?}", args); //FIXME: log using proper logging library
    
    let a = add(1, 2);
    println!("testnum {}", a);
    
}
