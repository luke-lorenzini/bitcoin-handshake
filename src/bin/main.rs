use std::default;
use std::error::Error;
use std::io::prelude::*;
use std::str::Bytes;
// use std::net::TcpStream;

use reqwest::Version;
use tokio::{io::AsyncReadExt, net::TcpStream};
use tokio::io::AsyncWriteExt;
// use std::error::Error;


use reqwest::{
    header::{ACCEPT, CONTENT_TYPE},
    Client,
};
// use tungstenite::{connect, http::response, Message};
// use tokio::sync::mpsc::{channel, Sender};
use url::{Url, ParseError};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[tokio::main]
async fn main() -> Result<()> {
    let message = b"F9BEB4D976657273696F6E0000000000550000002C2F86F37E1101000000000000000000C515CF6100000000000000000000000000000000000000000000FFFF2E13894A208D000000000000000000000000000000000000FFFF7F000001208D00000000000000000000000000";
    // println!("{:?}", message);

    // // let address = Url::parse("162.120.69.182:8333")?;
    let message = &thing();

    // Connect to a peer
    let mut stream = TcpStream::connect("162.120.69.182:8333").await?;
    // let mut stream = TcpStream::connect("217.230.42.55:8333").await?;
    // // let (mut reader, mut writer) = stream.split();
    
    // Write some data.
    stream.write_all(message).await?;
    println!("{:?}", stream);
    // // stream.write_all(message).await?;
    // // println!("{:?}", stream);
    let mut buffer = [0; 24];
    let len = stream.read(&mut buffer).await?;
    println!("reader: {:?}, buffer: {:X?}", len, buffer);

    // Response payload
    let mut buffer = [0; 66];
    let len = stream.read(&mut buffer).await?;
    println!("reader: {:?}, buffer: {:X?}", len, buffer);

    // Verack
    let mut buffer = [0; 20];
    let len = stream.read(&mut buffer).await?;
    println!("reader: {:?}, buffer: {:X?}", len, buffer);


    // let mut buffer = [0; 10];
    // // loop {
    //     // read up to 10 bytes
    //     let n = reader.read(&mut buffer[..]).await?;
    //     println!("The bytes: {:?}", &buffer[..n]);
    // // }

    // let (mut socket, response) = connect(address).expect("Can't connect");

    // let base_address = String::from("162.120.69.182");
    // let extension_address = String::from(":8333");
    // let address = format!("{}{}", base_address, extension_address);
    // println!("address: {:?}", address);

    // let client = Client::new();

    // let response = client
    //     .get(&format!("{}", &address))
    //     .header(CONTENT_TYPE, "application/json")
    //     .header(ACCEPT, "application/json")
    //     // .body("F9BEB4D976657273696F6E0000000000550000002C2F86F37E1101000000000000000000C515CF6100000000000000000000000000000000000000000000FFFF2E13894A208D000000000000000000000000000000000000FFFF7F000001208D00000000000000000000000000")
    //     .send()
    //     .await
    //     .expect("Failed to execute request.");

    // match response.status() {
    //     reqwest::StatusCode::UNAUTHORIZED => println!("{}", response.status()),
    //     reqwest::StatusCode::OK => {
    //         // println!("response: {:?}", response);
    //         let message = response.bytes().await.unwrap();

    //         // tx.send(message.to_vec()).await.unwrap();
    //     }
    //     _ => println!("unhandled response: {}", response.status()),
    // }

    Ok(())
}

#[derive(Debug)]
struct Header {
    magic_bytes: [u8; 4],
    command:  [u8; 12],
    size:  [u8; 4],
    checksum: [u8; 4],
}

// #[derive(Debug)]
// enum Payload {
    
// }

#[derive(Debug)]
struct PayloadVersion {
    protocol_version : [u8; 4],//     │ 70014               │ little-endian              │       4 │ 7E 11 01 00                                     │
 services            : [u8; 8], // │ 0                   │ bit field, little-endian   │       8 │ 00 00 00 00 00 00 00 00                         │
 time                : [u8; 8],  //│ 1640961477          │ little-endian              │       8 │ C5 15 CF 61 00 00 00 00                         │
 remote_services      : [u8; 8], //│ 0                   │ bit field, little-endian   │       8 │ 00 00 00 00 00 00 00 00                         │
 remote_ip             : [u8; 16],//│ 46.19.137.74        │ ipv6, big-endian           │      16 │ 00 00 00 00 00 00 00 00 00 00 FF FF 2E 13 89 4A │
 remote_port          : [u8; 2], //│ 8333                │ big-endian                 │       2 │ 20 8D                                           │
 local_services        : [u8; 8],//│ 0                   │ bit field, little-endian   │       8 │ 00 00 00 00 00 00 00 00                         │
 local_ip             : [u8; 16], //│ 127.0.0.1           │ ipv6, big-endian           │      16 │ 00 00 00 00 00 00 00 00 00 00 FF FF 7F 00 00 01 │
 local_port           : [u8; 2], //│ 8333                │ big-endian                 │       2 │ 20 8D                                           │
 nonce                : [u8; 8],// │ 0                   │ little-endian              │       8 │ 00 00 00 00 00 00 00 00                         │
 user_agent           : [u8; 1],// │ ""                  │ compact size, ascii        │ compact │ 00                                              │
 last_block           : [u8; 4], //│ 0                   │ little-endian              │       4 │ 00 00 00 00    
}

// impl Payload::Version {
//     fn display_body(&self) {
//         let mut message: Vec<u32> = Vec::default();
        
//         self.protocol_version.iter().for_each(|f| message.push(*f));

//         println!("{:?}", message);
//     }
// }

#[derive(Debug)]
struct Message {
    header: Header,
    payload: PayloadVersion,
    message: Vec<u8>,
}

impl Message {
    fn display_header(&mut self) {
        // self.message: Vec<u8> = Vec::default();
        
        self.header.magic_bytes.iter().for_each(|f| self.message.push(*f));
        self.header.command.iter().for_each(|f| self.message.push(*f));
        self.header.size.iter().for_each(|f| self.message.push(*f));
        self.header.checksum.iter().for_each(|f| self.message.push(*f));

        // println!("{:X?}", self.message);
    }

    fn display_payload(&mut self) {
        // let mut message: Vec<u8> = Vec::default();
        
        self.payload.protocol_version.iter().for_each(|f| self.message.push(*f));
        self.payload.services.iter().for_each(|f| self.message.push(*f));
        self.payload.time.iter().for_each(|f| self.message.push(*f));
        self.payload.remote_services.iter().for_each(|f| self.message.push(*f));

        self.payload.remote_ip.iter().for_each(|f| self.message.push(*f));
        self.payload.remote_port.iter().for_each(|f| self.message.push(*f));
        self.payload.local_services.iter().for_each(|f| self.message.push(*f));
        self.payload.local_ip.iter().for_each(|f| self.message.push(*f));

        self.payload.local_port.iter().for_each(|f| self.message.push(*f));
        self.payload.nonce.iter().for_each(|f| self.message.push(*f));
        self.payload.user_agent.iter().for_each(|f| self.message.push(*f));
        self.payload.last_block.iter().for_each(|f| self.message.push(*f));

        // println!("{:X?}", self.message);
    }
}

fn thing() -> Vec<u8> {
    let message1 = b"F9BEB4D976657273696F6E0000000000550000002C2F86F37E1101000000000000000000C515CF6100000000000000000000000000000000000000000000FFFF2E13894A208D000000000000000000000000000000000000FFFF7F000001208D00000000000000000000000000";
    let message2 = b"F9BEB4D976657273696F6E0000000000550000002C2F86F37E1101000000000000000000C515CF6100000000000000000000000000000000000000000000FFFF2E13894A208D000000000000000000000000000000000000FFFF7F000001208D00000000000000000000000000";

    let header = Header {
        magic_bytes: [0xf9, 0xbe, 0xb4, 0xd9],
        command: ['v' as u8, 'e' as u8, 'r' as u8, 's'  as u8, 'i'  as u8, 'o'  as u8, 'n'  as u8, 0x0, 0x0, 0x0, 0x0, 0x0],
        size: [0x55, 0x00, 0x00, 0x00],
        checksum: [0x2c, 0x2f, 0x86, 0xf3],
    };
    println!("{:x?}", header.magic_bytes);
    println!("{:x?}", header.command);
    println!("{:x?}", header.size);
    println!("{:x?}", header.checksum);

    let payload = PayloadVersion { 
        protocol_version: [0x7E, 0x11, 0x01, 0x0] ,
        services: [0, 0, 0, 0, 0, 0, 0, 0],
        time: [0xC5, 0x15, 0xCF, 0x61, 0, 0, 0, 0],
        remote_services: [0, 0, 0, 0, 0, 0, 0, 0],
        remote_ip: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xFF, 0xFF, 0x2e, 0x13, 0x89, 0x4a],
        remote_port: [0x20, 0x8D  ],
        local_services: [0, 0, 0, 0, 0, 0, 0, 0 ],
        local_ip: [0, 0, 0, 0, 0, 0, 0, 0,0, 0, 0xFF, 0xFF, 0x7f, 0x0, 0, 0x01],
        local_port: [0x20, 0x8D ],
        nonce: [0, 0, 0, 0, 0, 0, 0, 0],
        user_agent: [0],
        last_block: [0, 0, 0, 0 ],
    };

    let mut message = Message {
        header,
        payload,
        message: Vec::default()
    };

    message.display_header();
    message.display_payload();
    println!("{:X?}", message.message);

    // let test_message = b"F9BEB4D976657273696F6E0000000000550000002C2F86F37E1101000000000000000000C515CF6100000000000000000000000000000000000000000000FFFF2E13894A208D000000000000000000000000000000000000FFFF7F000001208D00000000000000000000000000";
    // assert_eq!(message.message[0], test_message[0]);

    message.message
    // println!("{:?}", message);

}
