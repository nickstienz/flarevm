use crate::create_bytecode;

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
        #[allow(unused_attributes)]
        pub enum Bytecode {
            $($name = $hex,)*
        }

        #[allow(dead_code)]
        pub fn get_bytecode(hex: &str) -> Bytecode {
            match &format!("0x{}", hex)[..] {
                $(stringify!($hex) => Bytecode::$name,)*
                _ => panic!("Bytecode not find (WHAT)"),
            }
        }
    };
}
