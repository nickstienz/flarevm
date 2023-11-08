use crate::virtual_machine::VM;

#[derive(Debug)]
pub enum Error {
    InvalidBytecode(u8),
}

impl Error {
    pub fn panic(vm: &mut VM, err: Error) -> ! {
        vm.clear_registers();
        vm.unwind_stack(0);
        vm.clear_strings();

        eprintln!(
            "[!] Panic with `crate::error::Error::panic` called {:?}!\n\\__[ {}\n   \\_ Program safely exited the FVM! ]",
            err,
            "Panics aren't currently handled!",
        );

        std::process::exit(1);
    }

    pub fn abort() -> ! {
        std::process::abort();
    }
}
