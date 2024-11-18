use network::shared::packet::{Packet, PacketType};

pub mod network;

fn main() {
    (0..1000000).for_each(|_| {
        let _p = Packet::encode(PacketType::None, &[5]);
        // println!("{:?}", p);
    });

    // Encode
    // let p = Packet::encode(
    //     PacketType::None,
    //     &[0x01, 0x02, 0x03, 0x04, 0xA3, 0x17, 0xDC, 0x0A],
    // );
    // println!("===Original Packet===\n{:?}\n", p);
    //
    // // Decode
    // let dec = Packet::decode_packet(&p);
    // println!("===Packet Decoded===\n{:?}", dec);
}
