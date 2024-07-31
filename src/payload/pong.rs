use crate::payload::*;

#[derive(Clone, Debug, Default)]
pub struct PayloadPong {
    pub nonce: [u8; 8],
}

impl PayloadTrait for PayloadPong {
    fn get_command_string(&self) -> [u8; 12] {
        Commands::PongCommand.message_string()
    }

    fn get_payload(&self) -> Vec<u8> {
        let mut pl: Vec<u8> = Vec::default();

        pl.extend(self.nonce);

        pl
    }
}

impl PayloadPong {
    pub fn new(nonce: [u8; 8]) -> Self {
        PayloadPong { nonce }
    }
}
