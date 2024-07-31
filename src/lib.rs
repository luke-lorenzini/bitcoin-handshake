use std::time::{SystemTime, UNIX_EPOCH};

use sha2::{Digest, Sha256};

pub mod header;
pub mod magic_bytes;
pub mod message;
pub mod payload;

pub trait PayloadTrait {
    fn get_command_string(&self) -> [u8; 12];
    fn get_payload(&self) -> Vec<u8>;
}

use core::fmt::Debug;
impl Debug for dyn PayloadTrait {
    fn fmt(&self, _f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // write!(f, "Series{{{}}}", self.get_payload().iter().for_each(|f| print!()))
        // write!()
        Ok(())
    }
}

/// calculate_checksum
///
/// Calculates the double sha-256 of a message and returns a little-endian u32 representation of the first four bytes.
///
/// ```
/// use btc_p2p::{self, *};
/// use hex_literal::hex;
///
/// let inp = hex!("7E1101000000000000000000C515CF6100000000000000000000000000000000000000000000FFFF2E13894A208D000000000000000000000000000000000000FFFF7F000001208D00000000000000000000000000");
/// let result = calculate_checksum(&inp);
///
/// assert_eq!([0x2c, 0x2f, 0x86, 0xf3].to_vec(), result.to_le_bytes());
/// ```
///
pub fn calculate_checksum(inp: &[u8]) -> u32 {
    let mut hasher = Sha256::new();
    hasher.update(inp);
    let result = hasher.finalize();

    let mut hasher = Sha256::new();
    hasher.update(result);
    let result = hasher.finalize();

    u32::from_le_bytes([result[0], result[1], result[2], result[3]])
}

/// get_time
///
/// Get the current unix time in seconds
///
fn get_time() -> u64 {
    let start = SystemTime::now();
    start
        .duration_since(UNIX_EPOCH)
        .expect("88 miles per hour!")
        .as_secs()
}
