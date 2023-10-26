const REGISTER_NUMBER: usize = 4;
const STACK_SIZE: usize = 255;
const STRING_POOL_SIZE: usize = 255;

#[derive(Debug)]
pub struct VM {
    registers: [i64; REGISTER_NUMBER],
    stack: Vec<Item>,
    string_pool: Vec<String>,
}

#[derive(Debug)]
pub enum Item {
    Numberi32(i32),
    String(u32),
    None,
}

impl VM {
    pub fn new() -> Self {
        Self {
            registers: [0; REGISTER_NUMBER],
            stack: Vec::with_capacity(STACK_SIZE),
            string_pool: Vec::with_capacity(STRING_POOL_SIZE),
        }
    }

    #[inline]
    pub fn get_register(&self, register: usize) -> i64 {
        self.registers[register]
    }

    #[inline]
    pub fn set_register(&mut self, register: usize, value: i64) {
        if register > REGISTER_NUMBER {
            panic!("Out of bounds (register)");
        }
        self.registers[register] = value;
    }
}
