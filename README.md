# cjdns-rs [![Build Status](https://travis-ci.org/kpcyrd/cjdns-rs.svg?branch=master)](https://travis-ci.org/kpcyrd/cjdns-rs) [![Crates.io](https://img.shields.io/crates/v/cjdns.svg)](https://crates.io/crates/cjdns)

Admin API implementation of [cjdns].

[cjdns]: https://github.com/cjdelisle/cjdns

## Getting started

```rust
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

## License

LGPLv3
