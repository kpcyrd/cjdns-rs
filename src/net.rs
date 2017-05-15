//! Implementation of the udp socket
use std::io;
use std::net::UdpSocket;

pub trait CjdnsNetSocket {
    fn send(&self, buf: &[u8]) -> Result<usize, io::Error>;
    fn recv(&self) -> Result<Vec<u8>, io::Error>;
}


#[derive(Debug)]
pub struct CjdnsUdpSocket {
    socket: UdpSocket,
}

impl CjdnsUdpSocket {
    pub fn new(addr: &str) -> Result<CjdnsUdpSocket, io::Error> {
        let socket = UdpSocket::bind("127.0.0.1:43211")?;
        socket.connect(addr)?;
        Ok(CjdnsUdpSocket {
            socket: socket,
        })
    }
}

impl CjdnsNetSocket for CjdnsUdpSocket {
    fn send(&self, buf: &[u8]) -> Result<usize, io::Error> {
        let amt = self.socket.send(buf)?;
        Ok(amt)
    }

    fn recv(&self) -> Result<Vec<u8>, io::Error> {
        let mut buf = [0; 2048];
        let amt = self.socket.recv(&mut buf)?;

        let buf = &mut buf[..amt];
        Ok(buf.to_vec())
    }
}
