use crate::payload::*;

#[derive(Clone, Debug, Default)]
pub struct PayloadVerack;

impl PayloadTrait for PayloadVerack {
    fn get_command_string(&self) -> [u8; 12] {
        Commands::VerackCommand.message_string()
    }

    fn get_payload(&self) -> Vec<u8> {
        let pl: Vec<u8> = Vec::default();

        pl
    }
}

impl PayloadVerack {
    pub fn new() -> Self {
        PayloadVerack
    }
}
