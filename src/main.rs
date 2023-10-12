mod cli;

use cli::*;

fn main() {
    /*
    The FVM will need to handle some args so they are handled first.
    They are passed in to be pasrsed and then a Cli is returned with the
    right settings. Of course, this functionality needs to be
    implemented first :D Have fun!
    */
    let args: Vec<String> = std::env::args().collect();
    let cli = Cli::handle_args(&args);

    println!("{:?}", cli);
}
