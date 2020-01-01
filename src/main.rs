use std::io;
use std::str;
use std::thread;
use std::time::Duration;

use std::io::prelude::*;
use serial::prelude::*;
use std::io::BufReader;

fn main() {
    println!("Hello, world!");

    let mut port = serial::open("/dev/cu.usbserial-AL00WS14").unwrap();
    interact(&mut port).unwrap();
}

fn interact<T: SerialPort>(port: &mut T) -> io::Result<()> {
     port.reconfigure(&|settings| {
         settings.set_baud_rate(serial::Baud115200)?;
         settings.set_char_size(serial::Bits8);
         settings.set_parity(serial::ParityNone);
         settings.set_stop_bits(serial::Stop1);
         settings.set_flow_control(serial::FlowNone);
         Ok(())
     })?;

    port.set_timeout(Duration::from_millis(5000))?;

    println!("getting status...");
    println!("{}", get_status(port));

    println!("-----");
    println!("getting access points");
    println!("{}", list_available_aps(port));

    Ok(())
}

fn get_status<T: SerialPort>(port: &mut T) -> String {
    send(port, "AT+GMR\r\n")
}

fn list_available_aps<T: SerialPort>(port: &mut T) -> String {
    send(port, "AT+CWLAP\r\n")
}

fn send<T: SerialPort>(port: &mut T, command: &str) -> String {
    let request = command.as_bytes();

    port.write(&request).unwrap();
    port.flush().unwrap();

    read_all(port)
}

fn read_all<T: SerialPort>(port: &mut T) -> String {
    let mut buffer = vec![0u8; 256];
    let mut response : Vec<u8> = Vec::new();

    loop {
        let bytes_read = port.read(&mut buffer).unwrap();
        response.extend_from_slice(&buffer[..bytes_read]);

        let response_slice = &buffer[..bytes_read];
        if response_slice.ends_with(b"OK\r\n") || response_slice.ends_with(b"ERROR\r\n") {
            break;
        }
    }

    String::from_utf8(response).unwrap()
}
