use crate::components::error::Error;
use crate::types_to_enums;
use std::{mem, ptr};

const NUM_OF_REGISTERS: usize = 9;

#[derive(Debug)]
pub struct Registers {
    data: [u64; NUM_OF_REGISTERS],
    types: [RegisterTypes; NUM_OF_REGISTERS],
}

impl Registers {
    pub fn new() -> Self {
        Self {
            data: [0; NUM_OF_REGISTERS],
            types: [RegisterTypes::U8; NUM_OF_REGISTERS],
        }
    }

    pub fn get_register<T>(&self, reg: usize) -> T
    where
        T: Copy + TypeInfo,
    {
        self.safety_checks::<T>(reg);

        let value_type = T::type_of();
        let reg_type = self.types[reg];

        if value_type != reg_type {
            Error::panic(
                Error::TypesDontMatch,
                format!("Value({:?}) != Register({:?})", value_type, reg_type),
            )
        }

        let target_ptr = &self.data[reg] as *const u64;

        unsafe { *(target_ptr as *const T) }
    }

    pub fn set_register<T>(&mut self, reg: usize, value: T)
    where
        T: TypeInfo,
    {
        self.safety_checks::<T>(reg);

        self.types[reg] = T::type_of();

        let value_ptr = &value as *const T as *const u8;
        let target_ptr = &mut self.data[reg] as *mut u64 as *mut u8;
        let size = mem::size_of::<T>();

        unsafe {
            ptr::copy_nonoverlapping(value_ptr, target_ptr, size);
        }
    }

    fn safety_checks<T>(&self, reg: usize) {
        if reg >= NUM_OF_REGISTERS {
            Error::panic(
                Error::RegisterOutOfBounds,
                format!("Register {} is > {}", reg, NUM_OF_REGISTERS),
            );
        }

        let size = mem::size_of::<T>();

        if size > mem::size_of::<u64>() {
            Error::panic(
                Error::TypeSizeTooLarge,
                format!("Size({}) > U64({})", size, mem::size_of::<u64>()),
            );
        }
    }
}

types_to_enums!(
    i8 = I8,
    u8 = U8,
    i16 = I16,
    u16 = U16,
    i32 = I32,
    u32 = U32,
    i64 = I64,
    u64 = U64,
);

#[macro_export]
macro_rules! types_to_enums {
    (
        $($rtype:ty = $renum:ident,)*
    ) => {
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub enum RegisterTypes {
            $($renum,)*
        }

        pub trait TypeInfo {
            fn type_of() -> RegisterTypes;
        }

        $(
        impl TypeInfo for $rtype {
            fn type_of() -> RegisterTypes {
                RegisterTypes::$renum
            }
        }
        )*
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "RegisterOutOfBounds")]
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
    #[should_panic(expected = "TypesDontMatch")]
    fn get_register_i64_from_u8() {
        let r = Registers::new();
        let r0: i64 = r.get_register(0);
        assert_eq!(r0, 0i64);
    }

    #[test]
    #[should_panic(expected = "RegisterOutOfBounds")]
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
    fn set_register_i16() {
        let mut r = Registers::new();
        r.set_register::<i16>(0, 265);
        let r0: i16 = r.get_register(0);
        assert_eq!(r0, 265i16);
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
