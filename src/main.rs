use network::shared::packet::Packet;

pub mod network;

fn main() {
    cli_header();

    let p = network::shared::packet::Packet::new(
        network::shared::packet::PacketType::None,
        vec![0x01, 0x02, 0x03, 0x04, 0xA3, 0x17, 0xDC, 0x0A],
    );

    println!("Packet: {:#?}", p);
    println!("Packet encoded: {:?}", p.encode_packet());

    // Decode
    let dec = Packet::decode_packet(&p.encode_packet());
    println!("Packet decoded{:#?}", dec);
}

fn cli_header() {
    println!("================================================");
    println!("=== This program was made by Nicholas Stienz ===");
    println!("===  This software is under the MIT License  ===");
    println!("===------------------------------------------===");
    println!("===  !!! This program is alpha software !!!  ===");
    println!("===  !!!       Expect it to break       !!!  ===");
    println!("================================================");
    println!();
}
