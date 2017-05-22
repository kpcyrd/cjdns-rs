//! Definition of various message types
use std::io;
use rustc_serialize::Decodable;
use bencode::{self, encode, Decoder};

use errors::{Error, ApiError};

/// Decodes a bencoded Message
///
/// # Arguments
///
/// * `bytes` - A bencoded Message
///
/// # Example
///
/// ```
/// use cjdns::decode;
/// use cjdns::structs::Pong;
///
/// let msg = String::from("d000001:q4:ponge");
/// let bytes = msg.into_bytes();
///
/// let x: Pong = decode(bytes).unwrap();
/// println!("{:?}", x);
/// ```
pub fn decode<A>(bytes: Vec<u8>) -> Result<A, Error> where A: Decodable {
    let bencode: bencode::Bencode = bencode::from_vec(bytes)?;
    let mut decoder = Decoder::new(&bencode);
    let result: A = Decodable::decode(&mut decoder)?;
    Ok(result)
}

#[derive(Debug, RustcEncodable)]
pub struct CjdnsMsg {
    pub q: String,
    pub args: Option<CjdnsMsgArgs>,
}

impl CjdnsMsg {
    pub fn new(q: &str) -> CjdnsMsg {
        CjdnsMsg {
            q: q.to_owned(),
            args: None,
        }
    }

    pub fn new_with_args(q: &str, args: CjdnsMsgArgs) -> CjdnsMsg {
        CjdnsMsg {
            q: q.to_owned(),
            args: Some(args),
        }
    }

    pub fn with_args(&mut self, args: CjdnsMsgArgs) {
        self.args = Some(args);
    }

    pub fn encode(&self) -> io::Result<Vec<u8>> {
        encode(&self)
    }
}

#[derive(Debug, RustcEncodable)]
pub struct CjdnsMsgArgs {
    pub page: Option<u64>,
}

impl CjdnsMsgArgs {
    pub fn new() -> CjdnsMsgArgs {
        CjdnsMsgArgs {
            page: None,
        }
    }

    pub fn with_page(mut self, page: u64) -> CjdnsMsgArgs {
        self.page = Some(page);
        self
    }
}

#[derive(Debug, RustcDecodable)]
pub struct CjdnsPage {
    pub more: Option<u64>,
}

impl CjdnsPage {
    pub fn has_more(&self) -> bool {
        self.more.is_some()
    }
}

/// Translates api results to Result<_, _>
#[derive(Debug, RustcDecodable)]
pub struct CjdnsResult<T> {
    pub error: String,
    pub result: T,
}

impl<T> CjdnsResult<T> {
    /// Convert to Result<_, _>
    pub fn to_result(self) -> Result<T, ApiError> {
        match self.error.as_ref() {
            "none" => {
                Ok(self.result)
            },
            _ => {
                Err(ApiError::new(self.error))
            },
        }
    }
}

#[derive(Debug, RustcDecodable)]
pub struct Pong {
    pub q: String,
}

#[derive(Debug, RustcDecodable)]
pub struct PeerStats {
    pub peers: Vec<Peer>,
}

#[allow(non_snake_case)]
#[derive(Debug, RustcDecodable)]
pub struct Peer {
    pub addr: String,
    pub bytesIn: u64,
    pub bytesOut: u64,
    pub duplicates: u64,
    pub isIncoming: u64,
    pub last: u64,
    pub lostPackets: u64,
    pub receivedOutOfRange: u64,
    pub recvKbps: u64,
    pub sendKbps: u64,
    pub state: String,
    pub user: String,
}

#[allow(non_snake_case)]
#[derive(Debug, RustcDecodable)]
pub struct NodeStore {
    pub routingTable: Vec<Route>,
}

#[allow(non_snake_case)]
#[derive(Debug, RustcDecodable)]
pub struct Route {
    pub addr: String,
    pub bucket: u64,
    pub link: u64,
    pub time: u64,
}

#[allow(non_snake_case)]
#[derive(Debug, RustcDecodable)]
pub struct Node {
    bestParent: Parent,
    encodingScheme: Vec<EncodingScheme>,
    cost: u64,
    key: String,
    linkCount: u64,
    protocolVersion: u64,
    routeLabel: String,
}

#[allow(non_snake_case)]
#[derive(Debug, RustcDecodable)]
pub struct EncodingScheme {
    bitCount: u64,
    prefix: String,
    prefixLen: u64,
}

#[allow(non_snake_case)]
#[derive(Debug, RustcDecodable)]
pub struct Parent {
    ip: String,
    isOneHop: u64,
    parentChildLabel: String,
}
