use crate::{header::Header, magic_bytes::MagicBytes, message::Message};
use btc_p2p::{
    payload::{verack::PayloadVerack, version::PayloadVersion},
    *,
};

use std::{
    error::Error,
    mem,
    net::{Ipv4Addr, SocketAddr},
};

use bincode::deserialize;
use log::debug;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

const DEFAULT_REMOTE_IP: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
const DEFAULT_REMOTE_PORT: u16 = 8333;

async fn process_socket(stream: &mut TcpStream, socket_address: SocketAddr) -> Result<()> {
    let protocol_version = 70014;

    // Create the version message
    let payload = PayloadVersion::new(protocol_version, DEFAULT_REMOTE_IP, DEFAULT_REMOTE_PORT);
    let header = Header::new(
        MagicBytes::Mainnet,
        payload.get_command_string(),
        mem::size_of::<PayloadVersion>().try_into().unwrap(),
        calculate_checksum(&payload.get_payload()),
    );
    let message = Message::new(header, Box::new(payload));
    let header_size = mem::size_of::<Header>();
    let mut buffer = vec![0; header_size];
    stream.read_exact(&mut buffer).await?;
    debug!(
        "version response header: {:?}, some_vec: {:x?}",
        buffer.len(),
        buffer
    );
    println!(
        "Received 'version' command from {:?}, sending 'version' response.",
        socket_address
    );

    let rx_header: Header = deserialize(&buffer)?;
    let payload_size = u32::from_le_bytes(rx_header.size);
    buffer.resize(payload_size as usize, 0);
    stream.read_exact(&mut buffer).await?;
    debug!("buffer: {:?}", buffer);

    stream.write_all(&message.get_message()).await?;

    // Create the verack message
    let payload = PayloadVerack::new();
    let header = Header::new(
        MagicBytes::Mainnet,
        payload.get_command_string(),
        mem::size_of::<PayloadVersion>().try_into().unwrap(),
        calculate_checksum(&payload.get_payload()),
    );
    let message = Message::new(header, Box::new(payload));

    stream.write_all(&message.get_message()).await?;

    println!("'verack' sent, handshake complete!");

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let socket = format!("{}:{}", DEFAULT_REMOTE_IP, DEFAULT_REMOTE_PORT);
    let listener = TcpListener::bind(socket).await?;

    loop {
        let (mut stream, incoming_socket) = listener.accept().await?;
        process_socket(&mut stream, incoming_socket).await?;
    }
}
