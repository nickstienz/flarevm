use crate::create_packet_types;

// TODO: Most of the docs are out of date due to major updates!

/// The magic number that will not change. It will always be `0xAD01` and will
/// be used to identify that it is a packet made for the VM.
const MAGIC_NUMBER: u16 = 0xAD01;
/// Current version of the packet implementation
const VERSION: u8 = 0;
/// This is the size the checksum should be in bytes (u16). It is also used when
/// computing the `length` field of the packet so it must be consistant.
const CHECKSUM_SIZE: usize = 2;

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
    /// Pretty much all changes that change the way the data is read or written
    /// must have the version incremented.
    version: u8,
    /// An enum that detemines how to handle a packet. This enum will be
    /// converted to a number in the range 0-255 (8-bit) when assembling
    /// the final packet.
    p_type: PacketType,
    /// The length represents how many bytes come after the header.
    /// The length is computed by doing `data + checksum`.
    length: u16,
    // TODO: Fix `data` docs
    /// The data being passed around. It's a vector of 8-bit values.
    data: Box<[u8]>,
    /// The checksum is used to validate that the packet hasen't been corupted
    /// in any way.
    checksum: Option<u16>,
}

impl Packet {
    /// The `new()` function returns a new packet based on the given
    /// `PacketType` and data (`[u8]`). It can then be converted to binary
    /// for sending over the internet.
    pub fn new(p_type: PacketType, data: &[u8]) -> Self {
        // TODO: Add error handling to length
        let mut packet = Packet {
            magic_number: MAGIC_NUMBER,
            version: VERSION,
            p_type,
            length: (data.len() + CHECKSUM_SIZE) as u16,
            data: Box::from(data),
            checksum: None,
        };

        let partial = packet.prechecksum_encode_packet();
        packet.checksum = Some(Packet::calculate_checksum(&partial));
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
        data: Box<[u8]>,
        checksum: Option<u16>,
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

    fn prechecksum_encode_packet(&self) -> Vec<u8> {
        let header = self.generate_header();
        let total_size = header.len() + self.data.len() + CHECKSUM_SIZE;
        let mut bytes: Vec<u8> = Vec::with_capacity(total_size);

        bytes.extend_from_slice(&header);
        bytes.extend_from_slice(&self.data);
        bytes.extend_from_slice(&[0, 0]);
        bytes
    }

    /// The `encode_packet()` function will return a vector of `u8` that
    /// represents the original packet. This is used to send the data
    /// accross the network.
    ///
    /// This will convert any none numerical data types into numerical ones
    /// while organizing everything into the final structure.
    // TODO: Fix docs
    pub fn encode_packet(&self) -> Vec<u8> {
        let mut bytes = self.prechecksum_encode_packet();
        let len = bytes.len();

        match self.checksum {
            Some(checksum) => {
                bytes[len - CHECKSUM_SIZE..].copy_from_slice(&checksum.to_be_bytes());
                bytes
            }
            None => {
                bytes.extend_from_slice(&[0, 0]);
                let checksum = Packet::calculate_checksum(&bytes);
                let len = bytes.len();
                bytes[len - CHECKSUM_SIZE..].copy_from_slice(&checksum.to_be_bytes());
                bytes
            }
        }
    }

    // TODO: Add this function. It does not create a Packet struct and will
    // most likely go unused unless testing or special cases.
    pub fn quick_encode(p_type: PacketType, data: &[u8]) -> Box<[u8]> {
        todo!()
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

        // Grab all the data but leave the last two bytes as they are the checksum
        let data = Box::from(&packet[header_size..header_size + length as usize - CHECKSUM_SIZE]);

        // Compute and validate the checksum
        let calculated_checksum = Packet::calculate_checksum(packet);
        if calculated_checksum != 0 {
            return Err(PacketError::ChecksumIncorrect(calculated_checksum));
        }
        let packet_checksum = u16::from_be_bytes([
            packet[packet.len() - CHECKSUM_SIZE],
            packet[packet.len() - 1],
        ]);

        Ok(Packet::from_data(
            magic_number,
            version,
            p_type,
            length,
            data,
            Some(packet_checksum),
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
        let mut checksum: u32 = 0;

        for chunk in data.chunks(2) {
            let word = match chunk.len() {
                2 => (chunk[0] as u16) << 8 | (chunk[1] as u16),
                1 => (chunk[0] as u16) << 8,
                _ => 0,
            };
            checksum = checksum.wrapping_add(word as u32);
        }

        while checksum >> 16 != 0 {
            checksum = (checksum & 0xFFFF).wrapping_add(checksum >> 16);
        }

        !checksum as u16
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
    ChecksumIncorrect(u16),
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
        let p = Packet::new(PacketType::None, &[]);

        assert_eq!(
            p.encode_packet(),
            vec![0xAD, 0x01, 0x00, 0xFF, 0x00, 0x02, 81, 253]
        );
    }
}
