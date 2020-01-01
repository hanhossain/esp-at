use std::io;
use std::thread;
use std::time::Duration;

use std::io::prelude::*;
use serial::prelude::*;

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

//    println!("-----");
//    println!("getting access points");
//    println!("{}", list_available_aps(port));

    Ok(())
}

fn get_status<T: SerialPort>(port: &mut T) -> String {
    send(port, "AT+GMR\r\n")
}

fn list_available_aps<T: SerialPort>(port: &mut T) -> String {
    send(port, "AT+CWLAP\r\n")
}

fn send<T: SerialPort>(port: &mut T, command: &str) -> String {
    port.write(command.as_bytes()).unwrap();
    port.flush().unwrap();

    let mut buffer: Vec<u8> = vec![0u8; 256];
    let bytes_read = port.read(&mut buffer).unwrap();
    String::from_utf8(buffer).unwrap()
}