mod bytecode;
mod error;
mod stack;
mod string_pool;
mod virtual_machine;

use bytecode::Bytecode::*;
use error::Error;
use virtual_machine::*;

fn main() {
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

    let program: Vec<u8> = vec![0x01];

    let mut vm = VM::new(program);

    loop {
        let bytecode = vm.next_bytecode();

        match bytecode {
            exit => vm.exit(),
            abort => Error::abort(),
        }
    }
}
