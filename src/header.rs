use crate::magic_bytes::MagicBytes;

use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Header {
    pub magic_bytes: [u8; 4],
    pub command: [u8; 12],
    pub size: [u8; 4],
    pub checksum: [u8; 4],
}

impl Header {
    pub fn new(magic_bytes: MagicBytes, command: [u8; 12], size: u32, checksum: u32) -> Self {
        let magic_bytes = MagicBytes::get_magic_bytes(magic_bytes);
        Header {
            magic_bytes: magic_bytes.to_le_bytes(),
            command,
            size: size.to_le_bytes(),
            checksum: checksum.to_le_bytes(),
        }
    }

    pub fn get_header_message(&self) -> Vec<u8> {
        let mut pl: Vec<u8> = Vec::default();

        pl.extend(self.magic_bytes.iter().copied());
        pl.extend(self.command.iter().copied());
        pl.extend(self.size.iter().copied());
        pl.extend(self.checksum.iter().copied());

        pl
    }
}
