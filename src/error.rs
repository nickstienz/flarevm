#[derive(Debug)]
pub enum Error {
    InvalidBytecode,
    SliceTooSmall,
    SliceTooBig,
    ByteToStringError,
}

impl Error {
    pub fn panic(err: Error, msg: String) -> ! {
        eprintln!(
            "[!] Panic called with {:?}!\n\\__[ {}\n   \\_ FVM exited! ]",
            err, msg
        );

        std::process::exit(1);
    }

    pub fn abort() -> ! {
        std::process::abort();
    }
}
