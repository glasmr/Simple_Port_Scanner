mod ip_addr;
mod parse_address;

use std::net::Ipv4Addr;
use clap::Parser;
use crate::ip_addr::IpAddr;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    host: String,
    port: Option<String>,
}

fn main() {
    let args = Args::parse();


}
