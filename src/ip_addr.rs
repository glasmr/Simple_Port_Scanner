use std::fmt::Display;
use crate::ip_addr::IpAddr::{IPV4, IPV6};

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum IpAddr {
    IPV4(u8, u8, u8, u8),
    IPV6(u16, u16, u16, u16, u16, u16, u16, u16)
}

impl Display for IpAddr {
    fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            IpAddr::IPV4(a,b,c,d) => {
                write!(_f, "{a}.{b}.{c}.{d}")
            }
            IpAddr::IPV6(a, b, c, d, e, f, g, h) => {
                write!(_f, "{a:x}:{b:x}:{c:x}:{d:x}:{e:x}:{f:x}:{g:x}:{h:x}")
            }
        }
    }
}


pub struct Host {
    pub start: IpAddr,
    pub end: Option<IpAddr>,
    is_range: bool,
    current: Option<IpAddr>
}

#[allow(unused)]
impl Host {
    pub fn new(start: IpAddr) -> Host {
        Host {
            start,
            end: None,
            is_range: false,
            current: None
        }
    }

    pub fn range(start: IpAddr, end: IpAddr) -> Host {
        Host {
            start,
            end: Some(end),
            is_range: true,
            current: None
        }
    }

    #[allow(non_snake_case)]
    fn range_next(&mut self) -> Option<IpAddr> {
        if !self.is_range {return None};
        if self.current.is_none() {
            self.current = Some(self.start.clone());
            return self.current.clone();
        }
        if self.current == self.end {return None}
        let current = self.current.clone().unwrap();
        match current {
            IpAddr::IPV4(a, b, c, d) => {
                let (mut A, mut B, mut C, mut D) = (a, b, c, d);
                if D < 255 {
                    D += 1
                } else {
                    D = 0;
                    if C < 255 {
                        C += 1
                    } else {
                        C = 0;
                        if B < 255 {
                            B += 1
                        } else {
                            B = 0;
                            if A < 255 {
                                A += 1
                            }
                        }
                    }
                }
                self.current = Some(IPV4(A, B, C, D));
                return self.current.clone();
            }
            IpAddr::IPV6(a, b, c, d, e, f, g, h) => {
                let mut values: [u16; 8] = [a, b, c, d, e, f, g, h];
                for i in values.len() - 1..0 {
                    if values[i] < 65535 {
                        values[i] += 1;
                        break;
                    } else {values[i] = 0}
                }
                Some(IPV6(values[0], values[1], values[2], values[3], values[4], values[5], values[6], values[7]))
            }
        }
    }

    pub fn iter(&mut self) -> IterIp<'_> {
        IterIp {next: self}
    }
}

pub struct IterIp<'a> {
    next: &'a mut Host
}

impl<'a> Iterator for IterIp<'a> {
    type Item = IpAddr;
    fn next(&mut self) -> Option<IpAddr> {
        self.next.range_next()
    }
}

#[allow(dead_code)]
pub struct Port {
    pub start: u16,
    pub end: Option<u16>,
    current: Option<u16>
}

#[allow(unused)]
impl Port {
    pub fn new(start: u16) -> Port {
        Port {
            start,
            end: None,
            current: None
        }
    }
    pub fn range(start: u16, end: u16) -> Port {
        Port {
            start,
            end: Some(end),
            current: None
        }
    }
    fn range_next(&mut self) -> Option<u16> {
        if self.end.is_none() {return None};
        if self.current.is_none() {
            self.current = Some(self.start.clone());
            return self.current
        }
        if self.end == self.current {return None}
        let next_port = self.current.clone().unwrap() + 1;
        self.current = Some(next_port);
        self.current
    }

    pub fn iter(&mut self) -> IterPort<'_> {
        IterPort {next: self}
    }
}

pub struct IterPort<'a> {
    next: &'a mut Port
}

impl<'a> Iterator for IterPort<'a> {
    type Item = u16;
    fn next(&mut self) -> Option<u16> {
        self.next.range_next()
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::{
        Host,
        IpAddr::{IPV4},
        Port
    };
    #[test]
    fn test_ip_iter() {
        let mut range = Host::range(IPV4(192, 168, 1, 1), IPV4(192, 168, 1, 5));
        let mut iter = range.iter();
        assert_eq!(iter.next(), Some(IPV4(192, 168, 1, 1)));
        assert_eq!(iter.next(), Some(IPV4(192, 168, 1, 2)));
        assert_eq!(iter.next(), Some(IPV4(192, 168, 1, 3)));
        assert_eq!(iter.next(), Some(IPV4(192, 168, 1, 4)));
        assert_eq!(iter.next(), Some(IPV4(192, 168, 1, 5)));
        assert_eq!(iter.next(), None);
    }
    #[test]
    fn test_port_range() {
        let mut range = Port::range(1, 5);
        let mut iter = range.iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(4));
        assert_eq!(iter.next(), Some(5));
        assert_eq!(iter.next(), None);
    }
}