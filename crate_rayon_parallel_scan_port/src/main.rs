use std::net::{TcpStream, ToSocketAddrs};

use rayon::prelude::*;

fn main() {
    let mut addrs = "www.baidu.com:0".to_socket_addrs().unwrap();
    let base_addr = addrs.next().unwrap();
    (0..=65535_u16).into_par_iter().for_each(|i| {
        let mut addr = base_addr.clone();
        addr.set_port(i);
        let mark = if TcpStream::connect(addr).is_ok() {
            "open"
        } else {
            "closed"
        };
        println!("端口 {}: {}", i, mark);
    });
}
