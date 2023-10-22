mod bytecode;
mod vm;

use vm::VM;

fn main() {
    let mut vm = VM::new();
    println!("{:#?}", vm);
}
