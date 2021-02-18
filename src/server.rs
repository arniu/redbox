use std::io;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::net::TcpStream;

pub async fn run(addr: &str) -> io::Result<()> {
    let listener = TcpListener::bind(addr).await?;

    println!("Redbox started at {}", addr);
    while let Ok((stream, addr)) = listener.accept().await {
        println!("Accept from {}", addr);
        process(stream).await?;
    }
    println!("Redbox stopped");

    Ok(())
}

async fn process(mut stream: TcpStream) -> io::Result<()> {
    let mut buf = vec![0; 1024];

    while let Ok(n) = stream.read(&mut buf).await {
        if n > 0 {
            println!("Received {} bytes", n);
            stream.write_all(&buf[..n]).await?;
        }
    }

    Ok(())
}
