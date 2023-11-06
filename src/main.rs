mod bytecode;
mod stack;
mod string_pool;
mod virtual_machine;

use virtual_machine::*;

fn main() {
    let mut vm = VM::new();
    vm.set_register(IP, 28);
    vm.push(stack::StackItem::I32(5));
    vm.push(stack::StackItem::I32(7));
    vm.push_string("hello");
    vm.push_string("world");
    vm.push_string("hello");
    vm.push_string("hello");
    vm.push_string("hello");
    vm.push_string("hello");

    println!("{:#?}", vm);
}
