mod bytecode;
mod stack;
mod string_pool;
mod virtual_machine;

use virtual_machine::*;

fn main() {
    let vm = VM::new();
    println!("{:#?}", vm);
}
