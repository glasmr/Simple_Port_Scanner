use std::net::{Ipv4Addr, SocketAddr, TcpStream};
use std::time::Duration;
use crate::ip_addr::{Host, IpAddr, Port};
use crate::parse_address::ScanInfo;

pub fn port_scan(scan_info: &mut ScanInfo) {
    if scan_info.syn {
        syn_scan(scan_info.hosts.as_ref(), scan_info.ports.as_ref(), scan_info.timeout.clone().unwrap());
    } else {
        normal_scan(scan_info.hosts.as_mut(), scan_info.ports.as_mut(), scan_info.timeout.clone().unwrap());
    }
}

fn normal_scan(hosts: &mut Vec<Host>, ports: &mut Vec<Port>, timeout: u32) {
    //Loop over hosts
    for host in hosts {
        if host.is_range {
            for host_iter in host.iter() {
                println!("Scanning host: {}", host_iter);
                normal_check_open_ports(host_iter, ports, timeout);
            }
        } else {
            println!("Scanning host: {}", host.start);
            normal_check_open_ports(host.start.clone(), ports, timeout);
        }
    }
}

fn normal_check_open_ports(host: IpAddr, ports: &mut Vec<Port>, timeout: u32)  {
    //Loop over ports
    for port in ports {
        if port.is_range {
            for port_iter in port.iter() {
                normal_scan_port(host.clone(), port_iter, timeout);
            }
        } else {
            normal_scan_port(host.clone(), port.start.clone(), timeout);
        }
    }

}
fn normal_scan_port(host: IpAddr, port: u16, timeout: u32)  {
    let socket_addr;
    match host {
        IpAddr::IPV4(ip4) => {
            socket_addr = SocketAddr::from((ip4, port))
        }
        IpAddr::IPV6(ip6) => {
            socket_addr = SocketAddr::from((ip6, port))
        }
    }
    if let Ok(_stream) = TcpStream::connect_timeout(
        &socket_addr,
        Duration::from_millis(timeout as u64)) {
        println!("Port {port} is open");
    }
}

fn syn_scan(hosts: &Vec<Host>, ports: &Vec<Port>, timeout: u32) {unimplemented!()}