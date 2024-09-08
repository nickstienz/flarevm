use network::shared::packet::{Packet, PacketType};

pub mod network;

fn main() {
    let p = Packet::new(
        PacketType::None,
        vec![0x01, 0x02, 0x03, 0x04, 0xA3, 0x17, 0xDC, 0x0A],
    );

    println!("===Original Packet===\n{:#?}\n", p);
    println!("Packet encoded: {:?}\n", p.encode_packet());

    // Decode
    let dec = Packet::decode_packet(&p.encode_packet());
    println!("===Packet Decoded===\n{:#?}", dec);
}
