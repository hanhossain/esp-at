use std::io::prelude::*;

pub struct ATClient<T> where T : Read + Write {
    handle: T,
}

impl<T: Read + Write> ATClient<T> {
    pub fn new(handle: T) -> Self {
        ATClient::<T> { handle }
    }

    pub fn get_status(&mut self) -> String {
        self.send("AT+GMR")
    }

    pub fn list_available_aps(&mut self) -> String {
        self.send("AT+CWLAP")
    }

    fn send(&mut self, command: &str) -> String {
        let command = command.to_owned() + "\r\n";
        let request = command.as_bytes();

        self.handle.write(&request).unwrap();
        self.handle.flush().unwrap();

        self.read_all()
    }

    fn read_all(&mut self) -> String {
        let mut buffer = vec![0u8; 256];
        let mut response: Vec<u8> = Vec::new();

        loop {
            let bytes_read = self.handle.read(&mut buffer).unwrap();
            response.extend_from_slice(&buffer[..bytes_read]);

            let response_slice = &buffer[..bytes_read];
            if response_slice.ends_with(b"OK\r\n") || response_slice.ends_with(b"ERROR\r\n") {
                break;
            }
        }

        String::from_utf8(response).unwrap()
    }
}
