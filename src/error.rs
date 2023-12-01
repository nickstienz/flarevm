#[derive(Debug)]
pub enum Error {
    InvalidBytecode,
    RustPanic,
}

impl Error {
    pub fn panic(err: Error, safe: bool, msg: String) -> ! {
        let exit_msg = if safe {
            String::from("FVM safely exited!")
        } else {
            String::from("FVM paniced!")
        };

        eprintln!(
            "[!] Panic called with {:?}!\n\\__[ {}\n   \\_ {} ]",
            err, msg, exit_msg
        );

        std::process::exit(1);
    }

    pub fn abort() -> ! {
        std::process::abort();
    }
}
