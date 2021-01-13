extern crate async_std;
extern crate futures;

use std::time::Duration;

use async_std::io;
use async_std::net::TcpListener;
use async_std::task;

use futures::stream::StreamExt;

#[async_std::main]
async fn main() {
    let addr = "127.0.0.1:16142";
    let listener = TcpListener::bind(addr).await.unwrap();

    let server = {
        async move {
            let mut incoming = listener.incoming();
            while let Some(conn) = incoming.next().await {
                match conn {
                    Err(e) => eprintln!("accept failed = {:?}", e),
                    Ok(mut stream) => {
                        task::spawn(async move {
                            let mut reader = stream.clone();
                            let mut writer = stream;
                            task::sleep(Duration::from_secs(8)).await;
                            match io::copy(&mut reader, &mut writer).await {
                                Ok(amt) => {
                                    println!("wrote {} bytes", amt);
                                }
                                Err(err) => {
                                    eprintln!("IO error {:?}", err);
                                }
                            }
                        });
                    }
                }
            }
        }
    };
    println!("Server running on localhost:16142");
    server.await;
}
