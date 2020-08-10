extern crate ftp;

use ftp::FtpStream;
use std::io::Cursor;
use std::str;

fn main() {
    println!("Hello rust-ftp!");
    // Create a connection to an FTP server and authenticate to it.
    let mut ftp_stream = FtpStream::connect("127.0.0.1:21").unwrap();
    let _ = ftp_stream.login("admin", "admin").unwrap();

    // Get the current directory that the client will be reading from and writing to.
    println!("Current directory: {}", ftp_stream.pwd().unwrap());

    // Change into a new directory, relative to the one we are currently in.
    let _ = ftp_stream.cwd("test_data").unwrap();

    // Retrieve (GET) a file from the FTP server in the current working directory.
    let remote_file = ftp_stream.simple_retr("rust.txt").unwrap();
    println!(
        "Read file with contents\n{}\n",
        str::from_utf8(&remote_file.into_inner()).unwrap()
    );

    // Store (PUT) a file from the client to the current working directory of the server.
    let mut reader = Cursor::new("Hello from the Rust \"ftp\" crate!".as_bytes());
    let _ = ftp_stream.put("hello-rustftp.txt", &mut reader);
    println!("Successfully upload hello-rustftp.txt");

    // Terminate the connection to the server.
    let _ = ftp_stream.quit();
}
