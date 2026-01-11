//parse host and port strings and return Host and Port structs

use std::net::Ipv4Addr;
use super::Args;
use crate::ip_addr::{Host, IpAddr, Port};
pub fn parse_arguments(args: &Args) -> (Vec<Host>, Vec<Port>) {
    let host_str: String = args.host.clone();
    let port_str: Option<String> = args.port.clone();
    unimplemented!()
}

fn parse_host_v4(host: String) -> Vec<Host> {
    let mut hosts: Vec<Host> = Vec::new();
    let host_list = host.split(",").collect::<Vec<&str>>();
    for host in host_list {
        if host.to_lowercase() == "localhost" {
            hosts.push(Host::new(IpAddr::IPV4(Ipv4Addr::LOCALHOST)));
        }
        let addr_chunks = host.split(".").collect::<Vec<&str>>();
        if addr_chunks.last().unwrap().contains("/") {
            //CIDR Notation xxx/xx
            let (start, end) = decode_cidr(addr_chunks);
            hosts.push(Host::range(IpAddr::IPV4(start), IpAddr::IPV4(end)));
        } else if addr_chunks.last().unwrap().contains("-") {
            //Range xxx-xxx
            let last: Vec<&str> = addr_chunks.last().unwrap().split('-').collect::<Vec<&str>>();
            let last_bytes: [u8; 2] = [last[0].parse::<u8>().unwrap(), last[1].parse::<u8>().unwrap()];
            if last_bytes[0] >= last_bytes[1] {panic!("Invalid host range")}
            hosts.push(Host::range(IpAddr::IPV4(Ipv4Addr::new(
                addr_chunks[0].to_string().parse::<u8>().unwrap(),
                addr_chunks[1].to_string().parse::<u8>().unwrap(),
                addr_chunks[2].to_string().parse::<u8>().unwrap(),
                last_bytes[0]
            )), IpAddr::IPV4(Ipv4Addr::new(
                addr_chunks[0].to_string().parse::<u8>().unwrap(),
                addr_chunks[1].to_string().parse::<u8>().unwrap(),
                addr_chunks[2].to_string().parse::<u8>().unwrap(),
                last_bytes[1]
            ))));
        } else {
            //Single address
            hosts.push(Host::new(IpAddr::IPV4(Ipv4Addr::new(
                addr_chunks[0].to_string().parse::<u8>().unwrap(),
                addr_chunks[1].to_string().parse::<u8>().unwrap(),
                addr_chunks[2].to_string().parse::<u8>().unwrap(),
                addr_chunks[3].to_string().parse::<u8>().unwrap(),
            ))));
        }
    }
    hosts
}

fn decode_cidr(addr: Vec<&str>) -> (Ipv4Addr, Ipv4Addr) {
    let last_byte = addr.last().unwrap().split('/').collect::<Vec<&str>>();
    let bitmask_bits = last_byte.last().unwrap().parse::<u32>().unwrap();
    let mask = u32::MAX << (32 - bitmask_bits);
    let host_mask = !mask;
    let bytes: [u8; 4] = [
        addr[0].parse::<u8>().unwrap(),
        addr[1].parse::<u8>().unwrap(),
        addr[2].parse::<u8>().unwrap(),
        last_byte[0].parse::<u8>().unwrap(),
    ];
    let ip_start = u32::from_be_bytes(bytes);
    let network_ip = ip_start & mask;
    let ip_end = network_ip | host_mask;

    (Ipv4Addr::from(ip_start), Ipv4Addr::from(ip_end))
}

fn parse_port(port: Option<String>) -> Vec<Port> {
    unimplemented!()
}