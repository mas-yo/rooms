
use async_std::net::TcpListener;
use async_std::prelude::*;
pub mod room;
pub mod net;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub async fn make_server() -> std::result::Result<(), Box<dyn std::error::Error>> {

    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    let mut incoming = listener.incoming();

    while let Some(stream) = incoming.next().await {
    println!("conn");
        let mut stream = stream?;
        stream.write_all(b"hello world").await?;
    }

    Ok(())
}