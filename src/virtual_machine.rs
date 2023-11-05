const REGISTER_NUMBER: usize = 2;
pub const IP: usize = 0;

#[derive(Debug)]
pub struct VM {
    registers: [i64; REGISTER_NUMBER],
}

impl VM {
    pub fn new() -> Self {
        Self {
            registers: [0; REGISTER_NUMBER],
        }
    }

    pub fn get_register(&self, register: usize) -> i64 {
        self.registers[register]
    }

    pub fn set_register(&mut self, register: usize, value: i64) {
        self.registers[register] = value;
    }
}
