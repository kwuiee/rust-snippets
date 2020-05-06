extern crate ssh2;

use ssh2::Session;
use std::io::prelude::*;
use std::net::TcpStream;
use std::path::Path;

fn main() {
    // Connect to the local SSH server
    let tcp = TcpStream::connect("127.0.0.1:22").unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    sess.userauth_password("username", "password").unwrap();

    assert!(sess.authenticated());

    // exec command
    let mut channel = sess.channel_session().unwrap();
    channel.exec("ls").unwrap();
    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();
    println!("{}", s);
    channel.wait_close().unwrap();
    println!("{}", channel.exit_status().unwrap());

    // upload file
    let mut remote_send = sess.scp_send(Path::new("remote"), 0o644, 10, None).unwrap();
    remote_send.write(b"1234567890").unwrap();
    remote_send.send_eof().unwrap();
    remote_send.wait_eof().unwrap();
    remote_send.wait_close().unwrap();
    println!("{}", remote_send.exit_status().unwrap());

    // download file
    let (mut remote_recv, stats) = sess.scp_recv(Path::new("remote")).unwrap();
    println!("remote file size: {}", stats.size());
    let mut contents = Vec::new();
    remote_recv.read_to_end(&mut contents).unwrap();
    println!("{:?}", contents);
    remote_recv.send_eof().unwrap();
    remote_recv.wait_eof().unwrap();
    remote_recv.wait_close().unwrap();
    println!("{}", remote_recv.exit_status().unwrap());
}
