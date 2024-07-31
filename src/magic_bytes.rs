// List of the publicly advertised Magic Bytes
pub enum MagicBytes {
    Mainnet,
    Regtest,
    Testnet3,
    Signet,
    Namecoin,
}

impl MagicBytes {
    pub fn get_magic_bytes(magic_bytes: MagicBytes) -> u32 {
        let res: [u8; 4] = match magic_bytes {
            MagicBytes::Mainnet => [0xf9, 0xbe, 0xb4, 0xd9],
            MagicBytes::Regtest => [0xfa, 0xbf, 0xb5, 0xda],
            MagicBytes::Testnet3 => [0x0b, 0x11, 0x09, 0x07],
            MagicBytes::Signet => [0x0a, 0x03, 0xcf, 0x40],
            MagicBytes::Namecoin => [0xf9, 0xbe, 0xb4, 0xfe],
        };

        u32::from_le_bytes(res)
    }
}
