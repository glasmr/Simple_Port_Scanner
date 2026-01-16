use etherparse::{ip_number, IpHeaders, Ipv4Header, PacketBuilder, PacketHeaders};
use socket2::{Domain, Protocol, SockAddr, Socket, Type};
use std::net::{Ipv4Addr, SocketAddrV4};
use std::time::{Duration, Instant};
use std::{mem, thread};
use std::thread::sleep;

pub fn scan_v4(ip: Ipv4Addr, port: u16, timeout: u32) -> bool {
    let src_ip: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
    let src_port = (&src_ip as *const Ipv4Addr as u16 % 16383) + 49152; //random port based in pointer
    let packet = build_packet_v4(ip, src_ip, src_port, port);

    let handle = thread::spawn(move || {response_listener(
        src_ip,
        ip,
        src_port,
        port,
        Duration::from_millis(timeout as u64),
    )});

    sleep(Duration::from_millis(10));
    socket_send_packet(packet, ip, port);

    handle.join().unwrap()
}
fn build_packet_v4(src_ip: Ipv4Addr, dst_ip: Ipv4Addr, src_port: u16, dst_port: u16) -> Vec<u8> {
    let packet_builder = PacketBuilder::ip(
        IpHeaders::Ipv4(
            Ipv4Header::new(
                0,
                64,
                ip_number::TCP,
                src_ip.octets(),
                dst_ip.octets(),
        ).unwrap(),
        Default::default())).tcp(
        src_port,
        dst_port,
        12345,
        1024
    ).syn();
    let payload: [u8;0] = [];
    let mut packet: Vec<u8> = Vec::<u8>::with_capacity(packet_builder.size(0));

    packet_builder.write(&mut packet, &payload).unwrap();
    packet
}

fn socket_send_packet(packet: Vec<u8>, dst_ip: Ipv4Addr, dst_port: u16) {
    let socket = Socket::new(Domain::IPV4, Type::RAW, Some(Protocol::TCP)).unwrap();
    socket.set_header_included_v4(true).unwrap();
    let sock_addr = SockAddr::from(SocketAddrV4::new(dst_ip, dst_port));

    socket.send_to(&packet, &sock_addr).unwrap();


}

pub fn response_listener(src_ip: Ipv4Addr, target_ip: Ipv4Addr, src_port: u16, target_port: u16, timeout: Duration) -> bool {
    let listener = Socket::new(Domain::IPV4, Type::RAW, Some(Protocol::TCP)).unwrap();
    listener.set_read_timeout(Some(timeout)).unwrap();

    let mut buf = [mem::MaybeUninit::<u8>::new(0u8); 1024];

    let now = Instant::now();

    while now + timeout <= Instant::now()  {
        let (bytes_read, _addr) = listener.recv_from(&mut buf).unwrap(); //TODO: if this doesnt read anything, it will block the loop, need solution
        let packet_slice = &buf[0..bytes_read];
        let packet_bytes: Vec<u8> = packet_slice.iter().map(|val| unsafe {val.assume_init()}).collect::<Vec<u8>>();
        let header: PacketHeaders = PacketHeaders::from_ip_slice(&packet_bytes).unwrap();

        //dbg!(&header);
        /*if header.clone().transport.unwrap().tcp().unwrap().source_port == src_port {
            println!("{} is connected", header.clone().transport.unwrap().tcp().unwrap().source_port);
            continue;
        }*/

        let to_ip = header.clone().net.unwrap().ipv4_ref().unwrap().0.destination;
        let from_ip = header.clone().net.unwrap().ipv4_ref().unwrap().0.source;

        let to_port = header.clone().transport.unwrap().tcp().unwrap().destination_port;
        let from_port = header.clone().transport.unwrap().tcp().unwrap().source_port;

        if to_ip == src_ip.octets() &&
            to_port == src_port &&
            from_ip == target_ip.octets() &&
            from_port == target_port {
            if header.clone().transport.unwrap().tcp().unwrap().ack == true &&
                header.clone().transport.unwrap().tcp().unwrap().syn == true {
                //dbg!(&header);
                return true;
            } else {
                return false;
            }
        }
    }
    false
}

