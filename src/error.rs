#[derive(Debug)]
pub enum Error {
    InvalidBytecode,
    RustPanic,
}

impl Error {
    pub fn panic(err: Error, msg: String) -> ! {
        eprintln!(
            "[!] Panic with `crate::error::Error::panic` called {:?}!\n\\__[ {}\n   \\_ FVM safely exited! ]",
            err,
            msg,
        );

        std::process::exit(1);
    }

    pub fn abort() -> ! {
        std::process::abort();
    }
}
