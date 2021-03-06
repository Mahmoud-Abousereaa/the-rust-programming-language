#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::process;

use minigrep::Config;
fn main() {
    let args: Vec<String> = env::args().collect();

    // let query = &args[1];
    // let filename = &args[2];
    let config = Config::new(&args).unwrap_or_else(|err|{
      eprintln!("Problem parsing arguments: {}", err);
      process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
      eprintln!("Problem reading the file: {}", e);
      process::exit(1);
    }
}