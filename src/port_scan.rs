use std::collections::HashMap;
use std::net::{SocketAddr, TcpStream};
use std::sync::Mutex;
use std::time::Duration;
use rayon::prelude::*;
use crate::ip_addr::{Host, IpAddr, Port};
use crate::parse_address::{ScanInfo, ScanType};

pub fn port_scan(scan_info: &mut ScanInfo) {
    let scan_result: HashMap<IpAddr, HashMap<u16, bool>>;
    let hosts = expand_hosts_list(scan_info.hosts.as_mut());
    let ports = expand_ports_list(scan_info.ports.as_mut());
    scan(hosts, ports, scan_info.timeout.clone().unwrap(), scan_info.scan_type);
}

fn expand_hosts_list(hosts: &mut Vec<Host>) -> Vec<IpAddr> {
    let mut host_vec: Vec<IpAddr> = Vec::new();
    for host in hosts {
        if host.is_range {
            for host_iter in host.iter() {
               host_vec.push(host_iter)
            }
        } else {
            host_vec.push(host.start.clone())
        }
    }
    host_vec
}
fn expand_ports_list(ports: &mut Vec<Port>) -> Vec<u16> {
    let mut port_vec: Vec<u16> = Vec::new();
    for port in ports {
        if port.is_range {
            for port_iter in port.iter() {
                port_vec.push(port_iter)
            }
        } else {
            port_vec.push(port.start.clone())
        }
    }
    port_vec
}

fn scan(hosts: Vec<IpAddr>, ports: Vec<u16>, timeout: u32, scan_type: ScanType) -> HashMap<IpAddr, HashMap<u16, bool>> {
    let mut result: HashMap<IpAddr, HashMap<u16, bool>> = HashMap::new();

    for host in hosts {
        let port_result: Mutex<HashMap<u16, bool>> = Mutex::new(HashMap::new());
        match scan_type {
            ScanType::Connect => {
                ports.par_iter().for_each(|port| {
                    let s_result = connect_scan_port(host.clone(), *port, timeout);
                    port_result.lock().unwrap().insert(*port, s_result);
                });
            }
            ScanType::Syn => {
                ports.par_iter().for_each(|port| {
                    let s_result = syn_scan_port(host.clone(), *port, timeout);
                    port_result.lock().unwrap().insert(*port, s_result);
                });
            }
            ScanType::UDP => {}
        }
        
        result.insert(host, port_result.into_inner().unwrap());
    }
    result
}


fn connect_scan_port(host: IpAddr, port: u16, timeout: u32) -> bool {
    let socket_addr;
    match host {
        IpAddr::IPV4(ip4) => {
            socket_addr = SocketAddr::from((ip4, port))
        }
        IpAddr::IPV6(ip6) => {
            socket_addr = SocketAddr::from((ip6, port))
        }
    }
    if let Ok(stream) = TcpStream::connect_timeout(
        &socket_addr,
        Duration::from_millis(timeout as u64)) {
        if let Ok(local) = stream.local_addr() {
            if let Ok(peer) = stream.peer_addr() {
                if local == peer {return false}
            }
        }
        return true
    }
    false
}


fn syn_scan_port(host: IpAddr, port: u16, timeout: u32) -> bool {unimplemented!()}