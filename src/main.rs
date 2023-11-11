mod bytecode;
mod error;
mod stack;
mod string_pool;
mod virtual_machine;
mod vm_info;

use bytecode::Bytecode::*;
use error::Error;
use virtual_machine::*;
use vm_info::VMInfo;

const VM_INFO: VMInfo = VMInfo {
    version: 0,
    experimental: true,
    name: "Flare Virtual Machine",
    authors: "Nicholas Stienz",
};

fn main() {
    let program: Vec<u8> = vec![0x00];

    let mut vm = VM::new(VM_INFO, program);

    loop {
        let bytecode = vm.next_bytecode();

        match bytecode {
            exit => vm.exit(),
            abort => Error::abort(),
        }
    }
}
