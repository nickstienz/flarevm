mod components;
mod utils;

fn main() {
    // This is the gate to hell. I wish you luck traveler.
    let program: Vec<u8> = vec![
        /* This prints "Hello, World!\n"
        pushs 15, "Hello, World!\n"
        print
        exit 0
        */
        0x04, 0x0E, 0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x2C, 0x20, 0x57, 0x6F, 0x72, 0x6C, 0x64, 0x21,
        0x0A, 0x05, 0x00, 0x00,
    ];
}
