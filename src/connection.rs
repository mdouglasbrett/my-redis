use tokio::net::TcpStream;
use mini_redis::{Frame, Result};

struct Connection {
    stream: TcpStream
}

impl Connection {
    // The None case here is if we get to EOF
    pub async fn read_frame(&mut self) -> Result<Option<Frame>> {
        todo!();
    }
    pub async fn write_frame(&mut self, frame: &Frame) -> Result<()> {
        todo!();
    }
}
