mod bytecode;
mod error;
mod stack;
mod string_pool;
mod virtual_machine;

use bytecode::Bytecode::*;
use error::Error;
use virtual_machine::*;

fn main() {
    let program: Vec<u8> = vec![0x00];

    let mut vm = VM::new(program);

    loop {
        let bytecode = vm.next_bytecode();

        match bytecode {
            exit => vm.exit(),
            abort => Error::abort(),
        }
    }
}
