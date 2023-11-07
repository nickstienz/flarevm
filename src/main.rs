mod bytecode;
mod error;
mod stack;
mod string_pool;
mod virtual_machine;

use bytecode::{get_bytecode, Bytecode::*};
use virtual_machine::*;

fn main() {
    let program: Vec<u8> = vec![0x00];

    let mut vm = VM::new(program);

    loop {
        let current_bytecode = get_bytecode(vm.value_at_ip());
        vm.add_i64_to_register(IP, 1);

        match current_bytecode {
            exit => vm.clean_exit(),
            abort => vm.abort(),
        }
    }
}
