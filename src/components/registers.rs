use std::mem;

const NUM_OF_REGISTERS: usize = 9;

#[derive(Debug)]
struct Registers {
    data: [u64; NUM_OF_REGISTERS],
}

impl Registers {
    pub fn new() -> Self {
        Self {
            data: [0; NUM_OF_REGISTERS],
        }
    }

    pub fn get_register<T: Default>(&self, reg: usize) -> T {
        if reg >= NUM_OF_REGISTERS {
            panic!("(HANDLE) Register OOB");
        }

        let size = mem::size_of::<T>();

        if size > mem::size_of::<u64>() {
            panic!("(HANDLE) Size > u64");
        }

        let target_ptr = &self.data[reg] as *const u64 as *const u8;

        let mut value: T = Default::default();
        let value_ptr = &mut value as *mut T as *mut u8;

        unsafe {
            value_ptr.copy_from_nonoverlapping(target_ptr, size);
        }

        value
    }

    pub fn set_register<T>(&mut self, reg: usize, value: T) {
        if reg >= NUM_OF_REGISTERS {
            panic!("(HANDLE) Register OOB");
        }

        let size = mem::size_of::<T>();

        if size > mem::size_of::<u64>() {
            panic!("(HANDLE) Size > u64");
        }

        let target_ptr = &mut self.data[reg] as *mut u64 as *mut u8;
        let value_ptr = &value as *const T as *const u8;

        unsafe {
            target_ptr.copy_from_nonoverlapping(value_ptr, size);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn get_register_oob() {
        let r = Registers::new();
        r.get_register::<u8>(100);
    }

    #[test]
    fn get_register_u8() {
        let r = Registers::new();
        let r0: u8 = r.get_register(0);
        assert_eq!(r0, 0u8);
    }

    #[test]
    fn get_register_u64() {
        let r = Registers::new();
        let r0: u64 = r.get_register(0);
        assert_eq!(r0, 0u64);
    }

    #[test]
    fn get_register_i64() {
        let r = Registers::new();
        let r0: i64 = r.get_register(0);
        assert_eq!(r0, 0i64);
    }

    #[test]
    #[should_panic]
    fn set_register_oob() {
        let mut r = Registers::new();
        r.set_register(100, 69);
    }

    #[test]
    fn set_register_u8() {
        let mut r = Registers::new();
        r.set_register::<u8>(0, 69);
        let r0: u8 = r.get_register(0);
        assert_eq!(r0, 69u8);
    }

    #[test]
    fn set_register_u64() {
        let mut r = Registers::new();
        r.set_register::<u64>(0, 420);
        let r0: u64 = r.get_register(0);
        assert_eq!(r0, 420u64);
    }

    #[test]
    fn set_register_i64() {
        let mut r = Registers::new();
        r.set_register::<i64>(0, 420);
        let r0: i64 = r.get_register(0);
        assert_eq!(r0, 420i64);
    }
}
