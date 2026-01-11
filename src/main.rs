mod ip_addr;
mod parse_address;

use std::net::Ipv4Addr;
use clap::Parser;
use crate::ip_addr::IpAddr;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    host: String,
    port: String,
}

fn main() {
    //let args = Args::parse();

    let bytes: [u8; 4] = [192, 168, 0, 1];
    let mask = u32::MAX << (32 - 24);
    let ip = u32::from(Ipv4Addr::new(bytes[0], bytes[1], bytes[2], bytes[3]));
    let network = ip & mask;
    let net_addr = Ipv4Addr::from(network);
    println!("{:?}", net_addr);
}
