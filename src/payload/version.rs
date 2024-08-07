use crate::payload::*;

#[derive(Clone, Debug, Default)]
pub struct PayloadVersion {
    pub protocol_version: [u8; 4],
    pub services: [u8; 8],
    pub time: [u8; 8],
    pub remote_services: [u8; 8],
    pub remote_ip: [u8; 16],
    pub remote_port: [u8; 2],
    pub local_services: [u8; 8],
    pub local_ip: [u8; 16],
    pub local_port: [u8; 2],
    pub nonce: [u8; 8],
    pub user_agent: [u8; 1],
    pub last_block: [u8; 4],
}

impl PayloadTrait for PayloadVersion {
    fn get_command_string(&self) -> [u8; 12] {
        Commands::VersionCommand.message_string()
    }

    fn get_payload(&self) -> Vec<u8> {
        let mut pl: Vec<u8> = Vec::default();

        pl.extend(self.protocol_version.iter().copied());
        pl.extend(self.services.iter().copied());
        pl.extend(self.time.iter().copied());
        pl.extend(self.remote_services.iter().copied());
        pl.extend(self.remote_ip.iter().copied());
        pl.extend(self.remote_port.iter().copied());
        pl.extend(self.local_services.iter().copied());
        pl.extend(self.local_ip.iter().copied());
        pl.extend(self.local_port.iter().copied());
        pl.extend(self.nonce.iter().copied());
        pl.extend(self.user_agent.iter().copied());
        pl.extend(self.last_block.iter().copied());

        pl
    }
}

impl PayloadVersion {
    pub fn new(protocol_version: u32, remote_ip: Ipv4Addr, remote_port: u16) -> Self {
        PayloadVersion {
            protocol_version: protocol_version.to_le_bytes(),
            services: [0, 0, 0, 0, 0, 0, 0, 0],
            time: super::get_time().to_le_bytes(),
            remote_services: [0, 0, 0, 0, 0, 0, 0, 0],
            remote_ip: remote_ip.to_ipv6_mapped().octets(),
            remote_port: remote_port.to_be_bytes(),
            local_services: [0, 0, 0, 0, 0, 0, 0, 0],
            local_ip: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xff, 0xff, 0x7f, 0x0, 0, 0x01],
            local_port: [0x20, 0x8d],
            nonce: [0, 0, 0, 0, 0, 0, 0, 0],
            user_agent: [0],
            last_block: [0, 0, 0, 0],
        }
    }
}
