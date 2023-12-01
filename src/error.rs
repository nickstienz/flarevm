#[derive(Debug)]
pub enum Error {
    InvalidBytecode,
    RustPanic,
}

impl Error {
    pub fn panic(err: Error, msg: String) -> ! {
        eprintln!(
            "[!] Panic called {:?}!\n\\__[ {}\n   \\_ FVM safely exited! ]",
            err, msg,
        );

        std::process::exit(1);
    }

    pub fn abort() -> ! {
        std::process::abort();
    }
}
