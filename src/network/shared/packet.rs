use crate::create_packet_types;

/// The magic number that will not change. It will always be `0xAD01` and will
/// be used to identify that it is a packet made for the VM.
const MAGIC_NUMBER: u16 = 0xAD01;
/// Current version of the packet implementation
const VERSION: u8 = 0;
/// This is the size the checksum should be in bytes (u16). It is also used when computing the
/// `length` field of the packet so it must be consistant.
const CHECKSUM_SIZE: usize = 2;

/// The packet is used to send data between the server and client to
/// interact with the VM.
///
/// The packets will be sent using the TCP protocal.
///
/// The header is 6 bytes long and after that is the data and checksum.
#[derive(Debug)]
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
    checksum: u16,
}

impl Packet {
    /// The `new()` function returns a new packet based on the given
    /// `PacketType` and data (`[u8]`). It can then be converted to binary
    /// for sending over the internet.
    pub fn new(p_type: PacketType, data: Vec<u8>) -> Self {
        let mut p = Packet {
            magic_number: MAGIC_NUMBER,
            version: VERSION,
            p_type,
            length: (data.len() + CHECKSUM_SIZE) as u16,
            data,
            checksum: 0, // Temp value
        };

        let mut complete_data: Vec<u8> = p.generate_header();
        complete_data.extend_from_slice(&p.data);
        p.checksum = Packet::calculate_checksum(&complete_data);
        p
    }

    pub fn from_data(
        magic_number: u16,
        version: u8,
        p_type: PacketType,
        length: u16,
        data: Vec<u8>,
        checksum: u16,
    ) -> Self {
        Self {
            magic_number,
            version,
            p_type,
            length,
            data,
            checksum,
        }
    }

    /// The `encode_packet()` function will return a vector of `u8` that
    /// represents the original packet.
    ///
    /// This will convert any none numerical data types into numerical ones
    /// while organizing everything into the final structure.
    pub fn encode_packet(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = self.generate_header();

        if !self.data.is_empty() {
            bytes.extend_from_slice(&self.data);
        }

        bytes.extend_from_slice(&self.checksum.to_be_bytes());
        bytes
    }

    pub fn decode_packet(packet: &[u8]) -> Result<Packet, PacketError> {
        if packet.len() < 8 {
            return Err(PacketError::TooShort(packet.len()));
        }

        let magic_number = u16::from_be_bytes([packet[0], packet[1]]);
        if magic_number != MAGIC_NUMBER {
            return Err(PacketError::NotFVMPacket);
        }

        let version = packet[2];
        if version != VERSION {
            return Err(PacketError::WrongVersion(version));
        }

        let p_type = PacketType::from_u8(packet[3]);

        let header_size = 6;
        let length = u16::from_be_bytes([packet[4], packet[5]]);
        let total_len = header_size + length as usize;
        if packet.len() == total_len.to_be() {
            return Err(PacketError::LengthMismatch(packet.len(), total_len));
        }

        let data = packet[header_size..header_size + length as usize - CHECKSUM_SIZE].to_vec();

        let checksum = u16::from_be_bytes([packet[total_len - 2], packet[total_len - 1]]);
        let computed_checksum = Packet::calculate_checksum(&packet[..total_len - CHECKSUM_SIZE]);

        if checksum != computed_checksum {
            return Err(PacketError::ChecksumMismatch(checksum, computed_checksum));
        }

        Ok(Packet::from_data(
            magic_number,
            version,
            p_type,
            length,
            data,
            checksum,
        ))
    }

    fn generate_header(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        bytes.extend_from_slice(&self.magic_number.to_be_bytes());
        bytes.push(self.version);
        bytes.push(self.p_type as u8);
        bytes.extend_from_slice(&self.length.to_be_bytes());
        bytes
    }

    pub fn calculate_checksum(data: &[u8]) -> u16 {
        assert_eq!(CHECKSUM_SIZE, 2); // Check size of ChecksumType

        let mut checksum: u32 = 0;

        for chunk in data.chunks(2) {
            let word = match chunk.len() {
                2 => (chunk[0] as u16) << 8 | (chunk[1] as u16),
                1 => (chunk[0] as u16) << 8,
                _ => 0,
            };
            checksum = checksum.wrapping_add(word as u32);
        }

        checksum = (checksum & 0xFFFF) + (checksum >> 16);
        !checksum as u16
    }
}

create_packet_types!(
    /// The `None` type is used for testing and is not recognized internally
    /// so it will be skipped or throw an error all depending on the
    /// implementation.
    None = 0xFF,
);

#[macro_export]
macro_rules! create_packet_types {
    (
        $(
            $(#[$meta:meta])*
            $name:ident = $value:expr
        ),* $(,)?
    ) => {
        /// The `PacketType` is used to detemine what actions the VM must take
        /// to interpret the data being passed to it.
        ///
        /// All types are given a number in the `u8` range `(0-255 or 0x00-0xFF)` used
        /// to identify it in the final packet. This number should not be changed
        /// unless necessary and if it is, the `VERSION` constant should be incremented
        /// to reflect that the change will break other versions.
        #[repr(u8)]
        #[derive(Debug, Clone, Copy)]
        pub enum PacketType {
            $(
                $(#[$meta])*
                $name = $value,
            )*
        }

        impl PacketType {
            pub fn from_u8(p_type: u8) -> Self {
                match p_type {
                    $(
                        $value => PacketType::$name,
                    )*
                    _ => panic!("Handle errors!"),
                }
            }
        }
    };
}

#[derive(Debug)]
pub enum PacketError {
    NotFVMPacket,
    TooShort(usize),
    WrongVersion(u8),
    LengthMismatch(usize, usize),
    ChecksumMismatch(u16, u16),
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_empty_packet_as_bytes() {
        let p = Packet::new(PacketType::None, Vec::new());

        assert_eq!(
            p.encode_packet(),
            vec![0xAD, 0x01, 0x00, 0xFF, 0x00, 0x02, 81, 253]
        );
    }
}
