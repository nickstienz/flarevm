#[derive(Debug)]
pub struct Cli;

impl Cli {
    pub fn handle_args(args: &[String]) -> Cli {
        for arg in args {
            println!("Arg: {}", arg);
        }

        Cli {}
    }
}
