mod bytecode;
mod error;
mod stack;
mod string_pool;
mod utils;
mod virtual_machine;

use bytecode::Bytecode::*;
use error::Error;
use utils::hex_to_int;
use virtual_machine::*;

fn main() {
    // This is the gate to hell. I wish you luck traveler.
    let program: Vec<u8> = vec![0x01, 0x01, 0xFE, 0x00, 0x00];

    let mut vm = VM::new(&program);

    loop {
        let bytecode = vm.get_bytecode();

        match bytecode {
            exit => vm.exit(),
            ldb => {
                let register = vm.next_byte() as usize;
                let byte = vm.next_byte() as i64;
                vm.set_register(register, byte);
            }
            ldw => {
                let register = vm.next_byte() as usize;
                let bytes = vm.next_n_bytes(2);
                let word: u16 = hex_to_int(bytes);
                vm.set_register(register, word as i64);
            }
            ldd => {
                let register = vm.next_byte() as usize;
                let bytes = vm.next_n_bytes(4);
                let double: u32 = hex_to_int(bytes);
                vm.set_register(register, double as i64);
            }
            nop => (),
            abort => Error::abort(),
        }

        vm.add_i64_to_register(IP, 1);
    }
}
