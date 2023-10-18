mod cli;
mod macros;

use cli::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let cli = Cli::handle_args(&args);
    println!("{:?}", cli); // TODO: Let's work on other stuff first
}
