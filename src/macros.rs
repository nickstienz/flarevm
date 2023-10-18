#[macro_export]
macro_rules! instructions {
    (
        $($name:ident => $hex:tt)*
    ) => {
        pub fn get_instruction_from_hex(value: &str) -> Instruction {
            match value {
                $($hex)
            }
        }
    };
}
