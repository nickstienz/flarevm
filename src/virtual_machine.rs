use crate::{
    stack::{Stack, StackItem},
    string_pool::StringPool,
};

// Registers (IP + 8xGPR)
const NUMBER_OF_REGISTERS: usize = 9;
pub const IP: usize = 0;

#[derive(Debug)]
pub struct VM {
    registers: [i64; NUMBER_OF_REGISTERS],
    stack: Stack,
    string_pool: StringPool,
}

impl VM {
    pub fn new() -> Self {
        Self {
            registers: [0; NUMBER_OF_REGISTERS],
            stack: Stack::new(),
            string_pool: StringPool::new(),
        }
    }

    pub fn get_register(&self, register: usize) -> i64 {
        self.registers[register]
    }

    pub fn set_register(&mut self, register: usize, value: i64) {
        self.registers[register] = value;
    }

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
}
