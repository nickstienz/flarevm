use crate::{
    bytecode::{self, Bytecode},
    error::Error,
    stack::{Stack, StackItem},
    string_pool::StringPool,
};

// Registers (IP + SP + 8xGPR)
const NUMBER_OF_REGISTERS: usize = 10;
pub const IP: usize = 0;
pub const SP: usize = 1;

#[derive(Debug)]
pub struct VM {
    program: Vec<u8>,
    registers: [i64; NUMBER_OF_REGISTERS],
    stack: Stack,
    string_pool: StringPool,
}

impl VM {
    pub fn new(program: Vec<u8>) -> Self {
        Self {
            program,
            registers: [0; NUMBER_OF_REGISTERS],
            stack: Stack::new(),
            string_pool: StringPool::new(),
        }
    }

    pub fn exit(&self) -> ! {
        std::process::exit(0);
    }

    pub fn panic(&mut self, err: Error) -> ! {
        Error::panic(self, err);
    }

    // Registers
    pub fn get_register(&self, register: usize) -> i64 {
        self.registers[register]
    }

    pub fn set_register(&mut self, register: usize, value: i64) {
        self.registers[register] = value;
    }

    pub fn add_i64_to_register(&mut self, register: usize, value: i64) {
        self.registers[register] += value;
    }

    pub fn next_bytecode(&mut self) -> Bytecode {
        let bytecode = self.program[self.get_register(IP) as usize];
        self.add_i64_to_register(IP, 1);
        match bytecode::get_bytecode(bytecode) {
            Ok(bc) => bc,
            Err(e) => self.panic(e),
        }
    }

    pub fn clear_registers(&mut self) {
        self.registers.iter_mut().for_each(|r| *r = 0);
    }

    // Stack
    pub fn push(&mut self, value: StackItem) {
        self.stack.push(value);
    }

    pub fn push_string(&mut self, string: &str) {
        let index = self.string_pool.intern(string.as_ref());
        self.stack.push(StackItem::String(index));
    }

    pub fn pop(&mut self) -> Option<StackItem> {
        self.stack.pop()
    }

    pub fn unwind_stack(&mut self, target_size: u32) {
        if target_size == 0 {
            self.stack.data.clear();
            return;
        }

        while self.stack.get_size() > target_size {
            self.stack.data.pop();
        }
    }

    // String Pool
    pub fn clear_strings(&mut self) {
        self.string_pool.clear();
    }
}
