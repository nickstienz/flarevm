#[derive(Debug)]
pub enum Error {
    InvalidBytecode,
    SliceTooSmall,
    SliceTooBig,
    ByteToString,
    RegisterOutOfBounds,
    TypeSizeTooLarge,
}

impl Error {
    pub fn panic(err: Error, msg: String) -> ! {
        panic!(
            "[!] Panic called with {:?}!\n\\__[ {}\n   \\_ FVM exited! ]",
            err, msg
        );
    }

    pub fn abort() -> ! {
        std::process::abort();
    }
}
