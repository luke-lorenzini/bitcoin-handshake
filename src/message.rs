use crate::{header::Header, *};

// A message is built with a header + payload. For Bitcoin, headers are standard which payloads can change.
#[derive(Debug)]
pub struct Message {
    pub header: header::Header,
    pub payload: Box<dyn PayloadTrait>,
    pub message: Vec<u8>,
}

impl Message {
    pub fn new(header: Header, payload: Box<dyn PayloadTrait>) -> Self {
        Message {
            header,
            payload,
            message: Vec::default(),
        }
    }

    // Return a contiguous vector of the header + payload for transmission on the wire.
    pub fn get_message(&self) -> Vec<u8> {
        let mut pl: Vec<u8> = Vec::default();

        pl.extend(self.header.get_header_message().iter().copied());
        pl.extend(self.payload.get_payload().iter().copied());

        pl
    }
}
