// Import generated Protobuf code

use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

mod sensor_message;
mod multi_sensor_messages;

use multi_sensor_messages::Message;
use multi_sensor_messages::MessageType;

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

            // Deserialize the Message wrapper using Protobuf
            let message = Message::decode(&message_data[..])?;

            // Handle the message (log, forward to bridge, etc.)
            println!("Received image: width: {}, height: {}, encoding: {}", message.width, message.height, message.encoding);

            // Handle messages based on their type
            // match message.type {
            //     Some(Message::) => {
            //         let image = message.image.unwrap(); 
            //         println!("Received image: width: {}, height: {}, encoding: {}", image.width, image.height, image.encoding);
            //     }
            //     Some(MessageType::LASER_SCAN) => {
            //         let laser_scan = message.laser_scan.unwrap(); 
            //         println!("Received laser scan: ranges: {:?}", laser_scan.ranges);
            //     }
            //     Some(MessageType::IMU) => {
            //         let imu = message.imu.unwrap();
            //         println!("Received IMU data: orientation: {:?}", imu.orientation);
            //     }
            //     None => println!("Invalid message type"),
            // }
            // ... (Potential bridge integration) ...

            socket.write_all("Hello from Rust!\n".as_bytes()).await?;
        });
    }
}
