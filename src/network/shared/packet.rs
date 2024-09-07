/// The magic number that will not change. It will always be `0xAD01` and will
/// be used to identify that it is a packet made for the VM.
pub const MAGIC_NUMBER: u16 = 0xAD01;
/// Current version of the packet implementation
pub const VERSION: u8 = 0;
/// This is the type the checksum will use. It is also used when computing the
/// `length` field of the packet so it must be consistant.
pub type ChecksumType = u16;

/// The packet is used to send data between the server and client to
/// interact with the VM.
///
/// The packets will be sent using the TCP protocal.
///
/// The header is 6 bytes long and after that is the data and checksum.
pub struct Packet {
    /// The magic number is used for packet identification and will
    /// always be `0xAD01`.
    magic_number: u16,
    /// The version will increment on packet implementation changes.
    /// This includes changes to the `PacketType` enum.
    version: u8,
    /// An enum that detemines how to handle a packet. This enum will be
    /// converted to a number in the range 0-255 (8-bit) when assembling
    /// the final packet.
    p_type: PacketType,
    /// The length represents how many bytes come after the header.
    /// The length is computed by doing `data + checksum`.
    length: u16,
    /// The data being passed around. It's a collection of 8-bit values.
    data: Vec<u8>,
    /// The checksum is used to validate that the packet hasen't been corupted
    /// in some way.
    checksum: ChecksumType,
}

impl Packet {
    /// The `new()` function returns a new packet based on the given
    /// `PacketType` and data (`[u8]`). It can then be converted to binary
    /// for sending over the internet.
    pub fn new(p_type: PacketType, data: Vec<u8>) -> Self {
        Packet {
            magic_number: MAGIC_NUMBER,
            version: VERSION,
            p_type,
            length: (data.len() + size_of::<ChecksumType>()) as u16,
            data,
            checksum: 0, // TODO: Compute checksum based on data
        }
    }

    /// The `get_bytes()` function will return a vector of `u8` that
    /// represents the original packet.
    ///
    /// This will convert any none numerical data types into numerical ones
    /// while organizing everything into the final structure.
    pub fn get_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        bytes.extend_from_slice(&self.magic_number.to_be_bytes());
        bytes.push(self.version);
        bytes.push(self.p_type as u8);
        bytes.extend_from_slice(&self.length.to_be_bytes());

        if !self.data.is_empty() {
            bytes.extend_from_slice(&self.data);
        }

        bytes.extend_from_slice(&self.checksum.to_be_bytes());
        bytes
    }
}

/// The `PacketType` is used to detemine what actions the VM must take
/// to interpret the data being passed to it.
///
/// All types are given a number in the `u8` range `(0-255 or 0x00-0xFF)` used
/// to identify it in the final packet. This number should not be changed
/// unless necessary and if it is, the `VERSION` constant should be incremented
/// to reflect that the change will break other versions.
#[repr(u8)]
#[derive(Clone, Copy)]
pub enum PacketType {
    /// The `None` type is used for testing and will be ignored by the VM.
    None = 0xFF,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_empty_packet_as_bytes() {
        let p = Packet::new(PacketType::None, Vec::new());

        assert_eq!(
            p.get_bytes(),
            vec![0xAD, 0x01, 0x00, 0xFF, 0x00, 0x02, 0x00, 0x00]
        );
    }
}