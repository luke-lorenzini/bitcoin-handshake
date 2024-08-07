use crate::*;

use std::net::Ipv4Addr;

pub mod ping;
pub mod pong;
pub mod verack;
pub mod version;

pub enum Commands {
    PingCommand,
    PongCommand,
    VerackCommand,
    VersionCommand,
}

impl Commands {
    pub fn message_string(&self) -> [u8; 12] {
        match self {
            Self::PingCommand => [
                b'p', b'i', b'n', b'g', b'i', b'o', b'n', 0x0, 0x0, 0x0, 0x0, 0x0,
            ],
            Self::PongCommand => [
                b'p', b'o', b'n', b'g', b'i', b'o', b'n', 0x0, 0x0, 0x0, 0x0, 0x0,
            ],
            Self::VerackCommand => [
                b'v', b'e', b'r', b'a', b'c', b'k', 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            ],
            Self::VersionCommand => [
                b'v', b'e', b'r', b's', b'i', b'o', b'n', 0x0, 0x0, 0x0, 0x0, 0x0,
            ],
        }
    }
}
