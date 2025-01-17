mod cli;

use invoicelib::add;

fn main() {
    cli::args::parse();
    let a = add(1, 2);
    println!("Hello, world! {}", a);
    
}
