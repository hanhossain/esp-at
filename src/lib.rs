pub mod basic;
pub mod wifi;

use std::io::prelude::*;
use serial::SystemPort;

pub struct ATClient {
    handle: SystemPort,
}

impl ATClient {
    pub fn new(handle: SystemPort) -> Self {
        ATClient { handle }
    }

    pub fn send(&mut self, command: &str) -> String {
        // write
        let command = command.to_owned() + "\r\n";
        let request = command.as_bytes();

        self.handle.write(&request).unwrap();
        self.handle.flush().unwrap();

        // read
        let mut buffer = vec![0u8; 256];
        let mut response: Vec<u8> = Vec::new();

        loop {
            let bytes_read = self.handle.read(&mut buffer).unwrap();
            response.extend_from_slice(&buffer[..bytes_read]);

            let response_slice = &buffer[..bytes_read];
            if response_slice.ends_with(b"OK\r\n") || response_slice.ends_with(b"ERROR\r\n") || response_slice.ends_with(b"WIFI DISCONNECT\r\n") {
                break;
            }
        }

        String::from_utf8(response).unwrap()
    }
}
