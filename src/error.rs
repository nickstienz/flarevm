use crate::virtual_machine::VM;

#[derive(Debug)]
pub enum Error {
    InvalidBytecode(u8),
}

impl Error {
    pub fn exit(vm: &mut VM, err: Error) -> ! {
        vm.unwind_stack(0);
        eprintln!("Error: {:?}", err);
        std::process::exit(1);
    }

    pub fn panic(err: Error) -> ! {
        eprintln!("Panic with `crate::error::Error::panic` called {:?}", err);
        std::process::exit(1);
    }
}
