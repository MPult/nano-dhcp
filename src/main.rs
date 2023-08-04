use tokio::net::UdpSocket;
use std::net::SocketAddr;
use dhcproto::v4::{Message, Decoder, Decodable};

fn main() {
    let server_ip = "0.0.0.0"; // Listen on all available interfaces
    let server_port = 67;
    let server_addr: SocketAddr = format!("{}:{}", server_ip, server_port).parse().unwrap();

    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            dhcp_server(server_addr).await;
        });
}

async fn dhcp_server(server_addr: SocketAddr) {
    // Create a UDP socket bound to the DHCP server IP and port
    let socket = UdpSocket::bind(&server_addr).await.unwrap();

    // Buffer to store incoming DHCP messages
    let mut buf = [0u8; 1500];

    println!("DHCP server listening on {}", server_addr);

    // Enter the main loop to receive and process DHCP messages
    loop {
        match socket.recv_from(&mut buf).await {
            Ok((bytes_received, client_addr)) => {
                // Process the received DHCP message here
                // You can parse the DHCP message and take appropriate actions based on the message type
                // Example: Parse the message using a function and print the bytes received
                message_router(&buf[..bytes_received]);
                println!("Received {} bytes from {}", bytes_received, client_addr);
            }
            Err(e) => {
                eprintln!("Error receiving DHCP message: {:?}", e);
            }
        }
    }
}

// Function to parse the DHCP message (you can expand this to handle different DHCP message types)
fn message_router(message: &[u8]) {
    // Implement DHCP message parsing logic here
    let decoded_message = Message::decode(&mut Decoder::new(&message));
    println!("DHCP MESSAGE: {:#?}", decoded_message);
}

