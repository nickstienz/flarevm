use crate::create_bytecode;
use crate::error::Error;

create_bytecode!(
    // VM Instructions
    exit = 0x00,
    // Reserved Instructions
    abort = 0xFF,
);

#[macro_export]
macro_rules! create_bytecode {
    (
        $($name:ident = $hex:tt,)*
    ) => {
        #[allow(non_camel_case_types)]
        pub enum Bytecode {
            $($name = $hex,)*
        }

        pub fn get_bytecode(hex: u8) -> Bytecode {
            match hex {
                $($hex => Bytecode::$name,)*
                _ => Error::panic(Error::InvalidBytecode(hex)),
            }
        }
    };
}
