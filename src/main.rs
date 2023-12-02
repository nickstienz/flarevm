mod bytecode;
mod error;
mod stack;
mod string_pool;
mod utils;
mod virtual_machine;

use bytecode::Bytecode::*;
use error::Error;
use utils::*;
use virtual_machine::*;

fn main() {
    // TODO: Rework this project bc OOP is hard man ;-;
    // This is the gate to hell. I wish you luck traveler.
    let program: Vec<u8> = vec![
        0x04, 0x0E, 0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x2C, 0x20, 0x57, 0x6F, 0x72, 0x6C, 0x64, 0x21,
        0x0A, 0x05, 0x00, 0x00,
    ];

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
                let word: u16 = hex_to_int(&bytes);
                vm.set_register(register, word as i64);
            }
            ldd => {
                let register = vm.next_byte() as usize;
                let bytes = vm.next_n_bytes(4);
                let double: u32 = hex_to_int(&bytes);
                vm.set_register(register, double as i64);
            }
            pushs => {
                let len = vm.next_byte() as usize;
                let bytes = vm.next_n_bytes(len);
                let s = hex_to_str(&bytes);
                vm.push_string(s);
            }
            print => {
                let sidx = match vm.pop().unwrap() {
                    stack::StackItem::String(s) => s,
                    _ => panic!("WTF?!?!?!?!"),
                };
                let s = vm.get_string(sidx);
                print!("{}", s);
            }
            nop => (),
            abort => Error::abort(),
        }

        vm.add_i64_to_register(IP, 1);
    }
}
