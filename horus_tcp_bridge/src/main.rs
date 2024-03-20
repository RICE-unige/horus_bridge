use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::error::Error;

mod horus_bridge;
use horus_bridge::Message; // Import your Protobuf generated code
use horus_bridge::MessageType;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New connection: {}", addr);

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            // Implement your message framing (e.g., length-prefixing)
            let n = socket.read(&mut buf).await?; 

            // Deserialize the Message wrapper using Protobuf
            let message = Message::decode(&buf[..n])?;

            // Handle messages based on their type
            match message.type {
                Some(MessageType::STRING) => {
                    let data = message.data.unwrap(); 
                    println!("Received string message: {}", data);
                }
                Some(MessageType::INVALID) | None => {
                    println!("Invalid message type");                
                }
            }

            // ... (Potential bridge integration) ...

            socket.write_all("Hello from Rust!\n".as_bytes()).await?; // Placeholder response 
        });
    }
}
