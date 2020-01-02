use std::str;
use std::time::Duration;

use std::io::prelude::*;
use serial::prelude::*;

fn main() {
    println!("Hello, world!");

    let mut port = serial::open("/dev/cu.usbserial-AL00WS14").unwrap();

    port.reconfigure(&|settings| {
        settings.set_baud_rate(serial::Baud115200)?;
        settings.set_char_size(serial::Bits8);
        settings.set_parity(serial::ParityNone);
        settings.set_stop_bits(serial::Stop1);
        settings.set_flow_control(serial::FlowNone);
        Ok(())
    }).unwrap();

    port.set_timeout(Duration::from_millis(5000)).unwrap();

    let mut at_client = ATClient::new(port);

    println!("getting status...");
    println!("{}", at_client.get_status());

    println!("-----");
    println!("getting access points");
    println!("{}", at_client.list_available_aps());
}

struct ATClient<T> where T : Read + Write {
    handle: T,
}

impl<T: Read + Write> ATClient<T> {
    fn new(handle: T) -> Self {
        ATClient::<T> { handle }
    }

    fn get_status(&mut self) -> String {
        self.send("AT+GMR")
    }

    fn list_available_aps(&mut self) -> String {
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
