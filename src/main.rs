mod bytecode;
mod error;
mod stack;
mod string_pool;
mod virtual_machine;

use bytecode::Bytecode::*;
use error::Error;
use virtual_machine::*;

fn main() {
    // The panic hook must be the first thing to happen...
    std::panic::set_hook(Box::new(|info| {
        Error::panic(
            Error::RustPanic,
            format!(
                "{}",
                info.payload()
                    .downcast_ref::<&str>()
                    .unwrap_or(&"Unknown Error")
            ),
        );
    }));

    // This is the gate to hell. I wish you luck traveler.
    let program: Vec<u8> = vec![0xFE, 0xFE, 0xFE, 0x00];

    let mut vm = VM::new(&program);

    loop {
        let bytecode = vm.get_bytecode();

        match bytecode {
            exit => vm.exit(0),
            nop => (),
            abort => Error::abort(),
        }

        vm.add_i64_to_register(IP, 1);
    }
}
