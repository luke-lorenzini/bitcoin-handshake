#[cfg(test)]
mod tests {
    use btc_p2p::{payload::version::PayloadVersion, *};

    use std::net::Ipv4Addr;

    use hex_literal::hex;
    use sha2::{Digest, Sha256};

    const PROTOCOL_VERSION: u32 = 70014;
    const REMOTE_IP: Ipv4Addr = Ipv4Addr::new(162, 120, 69, 182);
    const REMOTE_PORT: u16 = 8333;

    #[test]
    fn test_new_payload() {
        let payload = PayloadVersion::new(PROTOCOL_VERSION, REMOTE_IP, REMOTE_PORT);
        let command = payload.get_command_string();

        assert_eq!(
            [b'v', b'e', b'r', b's', b'i', b'o', b'n', 0x0, 0x0, 0x0, 0x0, 0x0,],
            command
        );
    }

    #[test]
    fn test_get_checksum() {
        let inp = hex!("7E1101000000000000000000C515CF6100000000000000000000000000000000000000000000FFFF2E13894A208D000000000000000000000000000000000000FFFF7F000001208D00000000000000000000000000");
        let result = calculate_checksum(&inp);

        assert_eq!([0x2c, 0x2f, 0x86, 0xf3].to_vec(), result.to_le_bytes());
    }

    #[test]
    fn test_hash() {
        let inp = hex!("7E1101000000000000000000C515CF6100000000000000000000000000000000000000000000FFFF2E13894A208D000000000000000000000000000000000000FFFF7F000001208D00000000000000000000000000");
        let out1 = hex!("05e353cb72abeee898dc1be8377b3a039e805075383e041f70235e3557877db3");
        let out2 = hex!("2c2f86f3c70b718a4bed17bc5586ae8f0cb19c2cc5b626160f94015dc41da7de");

        let mut hasher = Sha256::new();
        hasher.update(inp);
        let result = hasher.finalize();
        assert_eq!(out1, result[..]);

        let mut hasher = Sha256::new();
        hasher.update(result);
        let result = hasher.finalize();
        assert_eq!(out2, result[..]);
    }
}
