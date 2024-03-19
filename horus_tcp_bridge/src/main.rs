// Import generated Protobuf code
mod sensor_message;

use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New connection: {}", addr);

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            // Read message length prefix (replace with your framing scheme)
            // let mut len_bytes = [0; 4];
            // socket.read_exact(&mut len_bytes).await?;
            // let message_len = u32::from_be_bytes(len_bytes);

            // Read message length prefix (using length prefixing)
            let mut len_bytes = [0; 4];
            socket.read_exact(&mut len_bytes).await?;
            let message_len = u32::from_be_bytes(len_bytes); 

            // Read the entire message based on the length
            let mut message_data = vec![0; message_len as usize];
            socket.read_exact(&mut message_data).await?;

            // Deserialize the message using Protobuf
            let message = sensor_message::Image::decode(&message_data[..])?;

            // Handle the message (log, forward to bridge, etc.)
            println!("Received image: width: {}, height: {}, encoding: {}", message.width, message.height, message.encoding);

            // ... (Potential bridge integration) ...

            socket.write_all("Hello from Rust!\n".as_bytes()).await?;
        });
    }
}
