/*!
  Admin API implementation of cjdns

  # Example

  ```rust,ignore
  extern crate cjdns;

  use cjdns::structs::Peer;

  fn main() {
      let sock = cjdns::Socket::udp("127.0.0.1:11234").unwrap();

      let x = sock.ping().unwrap();
      println!("{:?}", x);

      let x = sock.peer_stats().unwrap();
      let peers: Vec<Peer> = x.into_iter()
                              .flat_map(|page| page.peers)
                              .collect();
      for peer in peers {
          println!("{:?}", peer);
      }
  }
  ```
*/
extern crate rustc_serialize;
pub extern crate bencode;

mod errors;
pub mod net;
pub mod structs;

use std::io;
pub use errors::{Error, ApiError};
use net::{CjdnsNetSocket, CjdnsUdpSocket};
pub use structs::decode;
use structs::{CjdnsMsg, CjdnsMsgArgs, CjdnsPage, CjdnsResult};

use rustc_serialize::Decodable;

/// The socket that is used to talk to the cjdroute api
#[derive(Debug)]
pub struct Socket {
    socket: CjdnsUdpSocket,
}

impl Socket {
    /// Create a new udp socket
    pub fn udp(addr: &str) -> Result<Socket, Error> {
        let socket = CjdnsUdpSocket::new(addr)?;
        Ok(Socket {
            socket: socket,
        })
    }

    /// Send a CjdnsMsg to the socket
    pub fn send(&self, msg: &CjdnsMsg) -> Result<usize, Error> {
        let buf = msg.encode()?;
        let amt = self.socket.send(buf.as_slice())?;
        Ok(amt)
    }

    /// Receive an object from the socket
    pub fn recv<T: Decodable>(&self) -> Result<T, Error> {
        let buf = self.socket.recv()?;
        let msg = decode(buf)?;
        Ok(msg)
    }

    pub fn send_raw(&self, buf: Vec<u8>) -> Result<usize, io::Error> {
        self.socket.send(buf.as_slice())
    }

    pub fn recv_raw(&self) -> Result<Vec<u8>, io::Error> {
        self.socket.recv()
    }

    /// Receive a paginated list from the api
    pub fn recv_all<T: Decodable>(&self, msg: &mut CjdnsMsg) -> Result<Vec<T>, Error> {
        let mut ctr = 0;
        let mut pages = Vec::new();

        loop {
            let args = CjdnsMsgArgs::new().with_page(ctr);
            msg.with_args(args);
            self.send(&msg)?;

            let buf = self.socket.recv()?;
            let page: CjdnsPage = decode(buf.clone())?;

            let obj = decode(buf)?;
            pages.push(obj);

            if !page.has_more() {
                break;
            }

            ctr += 1;
        }

        Ok(pages)
    }

    pub fn recv_result<T: Decodable>(&self) -> Result<T, Error> where T: std::fmt::Debug {
        let x: CjdnsResult<T> = self.recv()?;
        let result = x.to_result()?;
        Ok(result)
    }

    pub fn ping(&self) -> Result<structs::Pong, Error> {
        let msg = CjdnsMsg::new("ping");
        self.send(&msg)?;

        let obj = self.recv()?;
        Ok(obj)
    }

    pub fn interfacecontroller_peer_stats(&self) -> Result<structs::PeerStats, Error> {
        let msg = CjdnsMsg::new("InterfaceController_peerStats");
        self.send(&msg)?;

        let obj = self.recv()?;
        Ok(obj)
    }

    pub fn peer_stats(&self) -> Result<Vec<structs::PeerStats>, Error> {
        let mut msg = CjdnsMsg::new("InterfaceController_peerStats");
        let objs = self.recv_all(&mut msg)?;
        Ok(objs)
    }

    pub fn nodestore_dump_table(&self) -> Result<structs::NodeStore, Error> {
        let msg = CjdnsMsg::new("NodeStore_dumpTable");
        self.send(&msg)?;

        let obj = self.recv()?;
        Ok(obj)
    }

    pub fn dump_table(&self) -> Result<Vec<structs::NodeStore>, Error> {
        let mut msg = CjdnsMsg::new("NodeStore_dumpTable");
        let objs = self.recv_all(&mut msg)?;
        Ok(objs)
    }

    pub fn nodestore_node_for_addr(&self) -> Result<structs::Node, Error> {
        let msg = CjdnsMsg::new("NodeStore_nodeForAddr");
        self.send(&msg)?;
        let obj = self.recv_result()?;
        Ok(obj)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
