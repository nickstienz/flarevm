#[derive(Debug)]
pub enum Error {
    InvalidBytecode,
    RustPanic,
    SliceTooSmall,
    SliceTooBig,
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
