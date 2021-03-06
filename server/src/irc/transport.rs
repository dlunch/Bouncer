use async_std::{io::Result, net::TcpStream};
use futures::{io::BufReader, AsyncBufReadExt, AsyncWriteExt, Stream, StreamExt};

use super::message::Message;

#[derive(Clone)]
pub struct Transport {
    stream: TcpStream,
}

impl Transport {
    pub fn new(stream: TcpStream) -> Self {
        Self { stream }
    }

    pub fn stream(&self) -> impl Stream<Item = Message> + '_ {
        let reader = BufReader::new(&self.stream);

        reader.lines().map(move |x| Message::from_raw(x.unwrap()))
    }

    pub async fn send_message(&self, message: &Message) -> Result<()> {
        let mut stream = self.stream.clone();
        stream.write(message.raw().as_bytes()).await?;

        Ok(())
    }
}
