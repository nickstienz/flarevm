pub mod network;

fn main() {
    cli_header();
    let p = network::shared::packet::Packet::new(
        network::shared::packet::PacketType::None,
        vec![0x01, 0x02, 0x03],
    );

    println!("Packet 1: {:#?}", p);
    println!("Packet 1 encoded: {:?}", p.encode_packet());

    let p2 =
        network::shared::packet::Packet::new(network::shared::packet::PacketType::None, Vec::new());
    println!("Packet 2: {:#?}", p2);
    println!("Packet 2 encoded: {:?}", p2.encode_packet());
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
