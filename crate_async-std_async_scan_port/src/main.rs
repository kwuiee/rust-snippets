extern crate async_std;

use std::error::Error;
use std::time::Duration;

use async_std::io;
use async_std::net::{SocketAddr, TcpStream, ToSocketAddrs};
use futures::future;

async fn connect(addr: SocketAddr) {
    let mark = if io::timeout(Duration::from_secs(20), TcpStream::connect(addr))
        .await
        .is_ok()
    {
        "open"
    } else {
        "closed"
    };
    println!("端口 {}: {}", addr.port(), mark);
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut addrs = "www.baidu.com:0".to_socket_addrs().await?;
    let base_addr = addrs.next().unwrap();
    let mut cons = vec![];
    (1000..=65535_u16).for_each(|i| {
        // vec![10250, 31847, 32288].into_iter().for_each(|i| {
        let mut addr = base_addr.clone();
        addr.set_port(i);
        cons.push(connect(addr));
    });
    future::join_all(cons).await;
    Ok(())
}
