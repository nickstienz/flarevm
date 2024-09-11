use crate::create_packet_types;

/// The magic number that will not change. It will always be `0xAD01` and will
/// be used to identify that it is a packet made for the VM.
const MAGIC_NUMBER: u16 = 0xAD01;
/// Current version of the packet implementation
const VERSION: u8 = 0;
/// This is the size the checksum should be in bytes (u16). It is also used when
/// computing the `length` field of the packet so it must be consistant.
const CHECKSUM_SIZE: usize = 2;
/// The CRC-16 Table is a precomputed table for the CRC-16 algo in order to
/// speed up the computation of the checksum.
const CRC16_CCITT_TABLE: [u16; 256] = [
    0x0000, 0x1021, 0x2042, 0x3063, 0x4084, 0x50A5, 0x60C6, 0x70E7, 0x8108, 0x9129, 0xA14A, 0xB16B,
    0xC18C, 0xD1AD, 0xE1CE, 0xF1EF, 0x1231, 0x0210, 0x3273, 0x2252, 0x52B5, 0x4294, 0x72F7, 0x62D6,
    0x9339, 0x8318, 0xB37B, 0xA35A, 0xD3BD, 0xC39C, 0xF3FF, 0xE3DE, 0x2462, 0x3443, 0x0420, 0x1401,
    0x64E6, 0x74C7, 0x44A4, 0x5485, 0xA56A, 0xB54B, 0x8528, 0x9509, 0xE5EE, 0xF5CF, 0xC5AC, 0xD58D,
    0x3653, 0x2672, 0x1611, 0x0630, 0x76D7, 0x66F6, 0x5695, 0x46B4, 0xB75B, 0xA77A, 0x9719, 0x8738,
    0xF7DF, 0xE7FE, 0xD79D, 0xC7BC, 0x48C4, 0x58E5, 0x6886, 0x78A7, 0x0840, 0x1861, 0x2802, 0x3823,
    0xC9CC, 0xD9ED, 0xE98E, 0xF9AF, 0x8948, 0x9969, 0xA90A, 0xB92B, 0x5AF5, 0x4AD4, 0x7AB7, 0x6A96,
    0x1A71, 0x0A50, 0x3A33, 0x2A12, 0xDBFD, 0xCBDC, 0xFBBF, 0xEB9E, 0x9B79, 0x8B58, 0xBB3B, 0xAB1A,
    0x6CA6, 0x7C87, 0x4CE4, 0x5CC5, 0x2C22, 0x3C03, 0x0C60, 0x1C41, 0xEDAE, 0xFD8F, 0xCDEC, 0xDDCD,
    0xAD2A, 0xBD0B, 0x8D68, 0x9D49, 0x7E97, 0x6EB6, 0x5ED5, 0x4EF4, 0x3E13, 0x2E32, 0x1E51, 0x0E70,
    0xFF9F, 0xEFBE, 0xDFDD, 0xCFFC, 0xBF1B, 0xAF3A, 0x9F59, 0x8F78, 0x9188, 0x81A9, 0xB1CA, 0xA1EB,
    0xD10C, 0xC12D, 0xF14E, 0xE16F, 0x1080, 0x00A1, 0x30C2, 0x20E3, 0x5004, 0x4025, 0x7046, 0x6067,
    0x83B9, 0x9398, 0xA3FB, 0xB3DA, 0xC33D, 0xD31C, 0xE37F, 0xF35E, 0x02B1, 0x1290, 0x22F3, 0x32D2,
    0x4235, 0x5214, 0x6277, 0x7256, 0xB5EA, 0xA5CB, 0x95A8, 0x8589, 0xF56E, 0xE54F, 0xD52C, 0xC50D,
    0x34E2, 0x24C3, 0x14A0, 0x0481, 0x7466, 0x6447, 0x5424, 0x4405, 0xA7DB, 0xB7FA, 0x8799, 0x97B8,
    0xE75F, 0xF77E, 0xC71D, 0xD73C, 0x26D3, 0x36F2, 0x0691, 0x16B0, 0x6657, 0x7676, 0x4615, 0x5634,
    0xD94C, 0xC96D, 0xF90E, 0xE92F, 0x99C8, 0x89E9, 0xB98A, 0xA9AB, 0x5844, 0x4865, 0x7806, 0x6827,
    0x18C0, 0x08E1, 0x3882, 0x28A3, 0xCB7D, 0xDB5C, 0xEB3F, 0xFB1E, 0x8BF9, 0x9BD8, 0xABBB, 0xBB9A,
    0x4A75, 0x5A54, 0x6A37, 0x7A16, 0x0AF1, 0x1AD0, 0x2AB3, 0x3A92, 0xFD2E, 0xED0F, 0xDD6C, 0xCD4D,
    0xBDAA, 0xAD8B, 0x9DE8, 0x8DC9, 0x7C26, 0x6C07, 0x5C64, 0x4C45, 0x3CA2, 0x2C83, 0x1CE0, 0x0CC1,
    0xEF1F, 0xFF3E, 0xCF5D, 0xDF7C, 0xAF9B, 0xBFBA, 0x8FD9, 0x9FF8, 0x6E17, 0x7E36, 0x4E55, 0x5E74,
    0x2E93, 0x3EB2, 0x0ED1, 0x1EF0,
];

/// The packet is used to send data between the server and client to
/// interact with the VM.
///
/// The structure of the packet is the same as the `Packet` struct.
/// Most of the data is coded to specifically use big endian format.
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
    /// The data being passed around. It's a vector of 8-bit values.
    data: Vec<u8>,
    /// The checksum is used to validate that the packet hasen't been corupted
    /// in any way.
    checksum: u16,
}

impl Packet {
    /// The `new()` function returns a new packet based on the given
    /// `PacketType` and data (`[u8]`). It can then be converted to binary
    /// for sending over the internet.
    pub fn new(p_type: PacketType, data: Vec<u8>) -> Self {
        let mut packet = Packet {
            magic_number: MAGIC_NUMBER,
            version: VERSION,
            p_type,
            length: (data.len() + CHECKSUM_SIZE) as u16,
            data,
            checksum: 0, // Temp value
        };

        let mut complete_data: Vec<u8> = packet.generate_header();
        complete_data.extend_from_slice(&packet.data);
        packet.checksum = Packet::calculate_checksum(&complete_data);
        packet
    }

    /// The `from_data()` function will create a new packet based on the
    /// provided data. Its primary use is constructing a new packet
    /// from a transported packet in the decoding stage.
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
    /// represents the original packet. This is used to send the data
    /// accross the network.
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

    /// The `decode_packet` function takes a slice of `u8` values and attempt
    /// to convert it into a new packet.
    ///
    /// This function has many stages in which it can error so the errors are
    /// sent back to the caller using the `Result` type. Code comments can be
    /// found in the source explaining the different stages to complete the
    /// decoding process.
    pub fn decode_packet(packet: &[u8]) -> Result<Packet, PacketError> {
        // Make sure the packet has at least a (header + checksum)
        if packet.len() < 8 {
            return Err(PacketError::TooShort(packet.len()));
        }

        // Make sure the packet belongs to the VM
        let magic_number = u16::from_be_bytes([packet[0], packet[1]]);
        if magic_number != MAGIC_NUMBER {
            return Err(PacketError::NotFVMPacket);
        }

        // Make sure the version is the same
        let version = packet[2];
        if version != VERSION {
            return Err(PacketError::WrongVersion(version));
        }

        // The error from this function is passed to the caller if the
        // `PacketType` does not exist based on the provided `u8`
        let p_type = PacketType::from_u8(packet[3])?;

        // This part has a lot to take in but it just checks to make sure
        // that the length of the (data + checksum) is valid or not.
        // It's important to note that the length is computed with both the
        // length of the data plus the checksum (data + checksum) so later
        // checks must account for that.
        let header_size = 6;
        let length = u16::from_be_bytes([packet[4], packet[5]]);
        let total_len = header_size + length as usize;
        if packet.len() != total_len {
            return Err(PacketError::LengthMismatch(packet.len(), total_len));
        }

        // Grab all the data but leave the last to bytes as they are the checksum
        let data = packet[header_size..header_size + length as usize - CHECKSUM_SIZE].to_vec();

        // Compute and validate the checksum
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

    /// The `generate_header()` function simply returns back the 6 byte header
    /// that makes up the final packet.
    fn generate_header(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        bytes.extend_from_slice(&self.magic_number.to_be_bytes());
        bytes.push(self.version);
        bytes.push(self.p_type as u8);
        bytes.extend_from_slice(&self.length.to_be_bytes());
        bytes
    }

    pub fn calculate_checksum(data: &[u8]) -> u16 {
        let mut crc: u16 = 0xFFFF;

        for byte in data {
            let lookup = ((crc >> 8) ^ (*byte as u16)) & 0xFF;
            crc = (crc << 8) ^ CRC16_CCITT_TABLE[lookup as usize];
        }

        println!(
            "CRC Data: {:?}\nCRC Final: {} ({:#018b})\nCRC 1s:    {} ({:#018b})",
            data, crc, crc, !crc, !crc
        );
        crc
    }
}

// The `PacketType` macro to handle the creation of the enum and `from_u8`
// function.
create_packet_types!(
    /// The `None` type is used for testing and is not recognized internally
    /// so it will be skipped or throw an error all depending on the
    /// implementation.
    None = 0xFF,
);

/// The `PacketError` enum is used in `Result` in order to inform other
/// parts of the VM about any errors that have happened internally to the
/// packet system. Some include data about the error while others don't.
#[derive(Debug)]
pub enum PacketError {
    /// This error handles when the packet does not match the `MAGIC_NUMBER`.
    NotFVMPacket,
    /// This error handles when the packet is less than 8 bytes long.
    /// This should never happen so long as the header and checksum are right.
    TooShort(usize),
    /// This error handles when the version fields don't match from the packet
    /// and VM. This stops the VM from handling older or newer packet types.
    WrongVersion(u8),
    /// This error handles when the length of the data does not match that of
    /// what the packet says it has. This can happen if the packet gets
    /// fragmented over the network or is just damaged.
    LengthMismatch(usize, usize),
    /// This error handles when the checksums do not match preventing any
    /// corrupted packets from being processed.
    ChecksumMismatch(u16, u16),
    /// This error handles when the `PacketType::to_u8` function cannot
    /// match the provided `u8` to a `PacketType`.
    InvalidPacketType(u8),
}

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
            /// The `from_u8` function converts a `u8` into a `PacketType`.
            ///
            /// This process is just a large `match` with all values possible.
            pub fn from_u8(p_type: u8) -> Result<Self, PacketError> {
                match p_type {
                    $(
                        $value => Ok(PacketType::$name),
                    )*
                    _ => Err(PacketError::InvalidPacketType(p_type)),
                }
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_checksum_size() {
        assert_eq!(CHECKSUM_SIZE, 2);
    }

    #[test]
    fn create_empty_packet_as_bytes() {
        let p = Packet::new(PacketType::None, Vec::new());

        assert_eq!(
            p.encode_packet(),
            vec![0xAD, 0x01, 0x00, 0xFF, 0x00, 0x02, 81, 253]
        );
    }
}
