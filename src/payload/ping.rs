use crate::payload::*;

#[derive(Clone, Debug, Default)]
pub struct PayloadPing {
    pub nonce: [u8; 8],
}

impl PayloadTrait for PayloadPing {
    fn get_command_string(&self) -> [u8; 12] {
        Commands::PingCommand.message_string()
    }

    fn get_payload(&self) -> Vec<u8> {
        let mut pl: Vec<u8> = Vec::default();

        pl.extend(self.nonce);

        pl
    }
}

impl PayloadPing {
    pub fn new(nonce: [u8; 8]) -> Self {
        PayloadPing { nonce }
    }
}
