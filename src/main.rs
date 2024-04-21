use tokio::net::UdpSocket;
use async_std::prelude::*;
use async_std::fs::File;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Race Trace");

    // Initialize the logger
    simple_logger::SimpleLogger::new().init().unwrap();

    // Bind the UDP socket to a local port
    let socket = UdpSocket::bind("127.0.0.1:7878").await?;
    println!("Listening on {}", socket.local_addr()?);

    let mut file = File::create("udp_log.txt").await?;

    loop {
        let mut buf = [0u8; 1024];
        match socket.recv_from(&mut buf).await {
            Ok((size, src)) => {
                let data = String::from_utf8_lossy(&buf[..size]);
                println!("Received from {}: {}", src, data); // Log to screen
                file.write_all(format!("Received from {}: {}\n", src, data).as_bytes()).await?; // Log to file
            },
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break;
            }
        }
    }

    Ok(())
}
