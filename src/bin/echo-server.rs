use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6142").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = vec![0; 1024];

            loop {
                match socket.read(&mut buf).await {
                    // @mdouglasbrett - as before, Ok(0) indicates a closed
                    // socket
                    Ok(0) => return,
                    Ok(n) => {
                        println!("GOT {:?}", &buf[..n]);
                        // @mdouglasbrett - normally I would attempt to check
                        // the other way around (for the Ok), I assume this is 
                        // more idiomatic
                        if socket.write_all(&buf[..n]).await.is_err() {
                            return;
                        }
                    }
                    Err(_) => return,
                }
            }
        });
    }
}
