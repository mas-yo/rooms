use async_std::net::TcpStream;
use async_std::prelude::*;
use async_std::task;

async fn client() -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
    stream.write_all(b"hello world").await?;

    let mut buf = vec![0u8; 1024];
    let n = stream.read(&mut buf).await?;

    Ok(())
}

fn main() {
    task::block_on(client());
}