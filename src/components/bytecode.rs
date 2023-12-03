use crate::components::error::Error;
use crate::create_bytecode;

create_bytecode!(
    // VM Instructions
    exit = 0x00,
    // Stack
    pushs = 0x04, // Push String
    // Output
    print = 0x05, // Print
    // Reserved Instructions
    nop = 0xFE,
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

        pub fn get_bytecode(hex: u8) -> Result<Bytecode, Error> {
            match hex {
                $($hex => Ok(Bytecode::$name),)*
                _ => Err(Error::InvalidBytecode),
            }
        }
    };
}
