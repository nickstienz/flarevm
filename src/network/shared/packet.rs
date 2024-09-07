/// Magic number that will not change
pub const MAGIC_NUMBER: u16 = 0xAD01;
/// Current version of the packet implementation
pub const VERSION: u8 = 0x00;

/// The packet used to send between the server and client that holds
/// data used to interact with the VM.
///
/// The header is 6 bytes long and after that is the data and checksum.
pub struct Packet {
    /// The magic number is used for packet identification and will always be 0xAD01
    magic_number: u16,
    /// The version will increment on packet implementation changes
    version: u8,
    /// An enum that detemine how to handle a packet
    p_type: u8, // TODO: Switch to enum
    /// The length represents how many bytes come after the header (data + checksum)
    length: u16,
    /// The data being passed around. It's a collection of 8-bit values.
    data: Vec<u8>,
    /// Used to validate that the packet hasen't been corupted in some way.
    checksum: u16,
}

impl Packet {
    pub fn new() -> Self {
        Packet {
            magic_number: MAGIC_NUMBER,
            version: VERSION,
            p_type: 0,        // TODO: Create packet type enum
            length: 0,        // TODO: Get length from data
            data: Vec::new(), // TODO: Get data
            checksum: 0,      // TODO: Compute checksum based on data
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_empty_packet_as_hex() {
        let p = Packet::new(PacketType::None, Vec::new());

        assert_eq!(p.get_hex(), "AD0100FF020000");
    }
}
