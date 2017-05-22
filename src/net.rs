//! Implementation of the udp socket
use std::io;
use std::net::UdpSocket;

pub trait CjdnsNetSocket {
    fn send(&self, buf: &[u8]) -> io::Result<usize>;
    fn recv(&self) -> io::Result<Vec<u8>>;
}


#[derive(Debug)]
pub struct CjdnsUdpSocket {
    socket: UdpSocket,
}

impl CjdnsUdpSocket {
    pub fn new(addr: &str) -> io::Result<CjdnsUdpSocket> {
        let socket = UdpSocket::bind("127.0.0.1:43211")?;
        socket.connect(addr)?;
        Ok(CjdnsUdpSocket {
            socket: socket,
        })
    }
}

impl CjdnsNetSocket for CjdnsUdpSocket {
    fn send(&self, buf: &[u8]) -> io::Result<usize> {
        let amt = self.socket.send(buf)?;
        Ok(amt)
    }

    fn recv(&self) -> io::Result<Vec<u8>> {
        let mut buf = [0; 2048];
        let amt = self.socket.recv(&mut buf)?;

        let buf = &mut buf[..amt];
        Ok(buf.to_vec())
    }
}
