use crate::{header::Header, magic_bytes::MagicBytes, message::Message, payload::Commands};
use btc_p2p::{payload::version::PayloadVersion, *};

use std::{env, error::Error, mem, net::Ipv4Addr, time::Duration};

use bincode::deserialize;
use log::{debug, info};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    time::timeout,
};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

const CONNECTION_TIMEOUT: u64 = 2;
const DEFAULT_REMOTE_IP: Ipv4Addr = Ipv4Addr::new(162, 120, 69, 182);
const DEFAULT_REMOTE_PORT: u16 = 8333;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        3 => {
            let remote_ip = args[1].parse::<Ipv4Addr>().unwrap_or(DEFAULT_REMOTE_IP);
            let remote_port = args[2].parse().unwrap_or(DEFAULT_REMOTE_PORT);
            run_handshake(remote_ip, remote_port).await
        }
        1 => run_handshake(DEFAULT_REMOTE_IP, DEFAULT_REMOTE_PORT).await,
        _ => Err(
            "Please enter zero or two parameters; ipv4 and port number, or no params for default"
                .into(),
        ),
    }?;

    Ok(())
}

async fn run_handshake(remote_ip: Ipv4Addr, remote_port: u16) -> Result<()> {
    // Step 1a, build up a payload for handshake
    let protocol_version = 70014;
    let payload = PayloadVersion::new(protocol_version, remote_ip, remote_port);

    // Step 1b, build up a header for handshake
    let header = Header::new(
        MagicBytes::Mainnet,
        payload.get_command_string(),
        mem::size_of::<PayloadVersion>().try_into().unwrap(),
        calculate_checksum(&payload.get_payload()),
    );

    let message = Message::new(header, Box::new(payload));

    // Connect to a peer
    let socket = format!("{}:{}", remote_ip, remote_port);
    println!("Attempting to connect: {:?}", socket);
    let mut stream = match timeout(
        Duration::from_secs(CONNECTION_TIMEOUT),
        TcpStream::connect(socket.clone()),
    )
    .await
    {
        Ok(ok) => ok,
        Err(e) => panic!("{}", format!("timeout while connecting to server : {}", e)),
    }?;
    println!("Successfully connected to: {:?}", socket);
    info!("{:x?}", message.get_message());
    stream.write_all(&message.get_message()).await?;

    let header_size = mem::size_of::<Header>();
    let mut rx_header: Header;
    let mut buffer = vec![0; header_size];

    stream.read_exact(&mut buffer).await?;
    info!(
        "version response header: {:?}, some_vec: {:x?}",
        buffer.len(),
        buffer
    );

    rx_header = deserialize(&buffer)?;
    debug!("deserialized header {:x?}", rx_header);

    if rx_header.command == Commands::VersionCommand.message_string() {
        println!("Received response to 'version' command, waiting for 'verack'");
    }

    let payload_size = u32::from_le_bytes(rx_header.size);
    buffer.resize(payload_size as usize, 0);

    // Response payload
    stream.read_exact(&mut buffer).await?;
    info!(
        "version response payload: {:?}, some_vec: {:x?}",
        buffer.len(),
        buffer
    );

    // Verack
    buffer.resize(header_size, 0);
    stream.read_exact(&mut buffer).await?;
    info!("verack: {:?}, some_vec: {:x?}", buffer.len(), buffer);
    rx_header = deserialize(&buffer)?;
    debug!("deserialized header {:?}", rx_header);

    if rx_header.command == Commands::VerackCommand.message_string() {
        println!("Received 'verack'");
    }

    stream.write_all(&rx_header.get_header_message()).await?;

    println!("Handshake successful!");

    Ok(())
}
