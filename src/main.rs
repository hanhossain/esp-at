use esp_at::ATClient;
use std::time::Duration;

use serial::prelude::*;
use esp_at::basic::ATBasic;

fn main() {
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
    let mut at_basic = ATBasic::new(at_client);

    println!("checking startup...");
    println!("{}", at_basic.test_startup());

    println!("getting status...");
    println!("{}", at_basic.get_status());

    println!("-----");
    println!("getting access points");
    println!("{}", at_basic.list_available_aps());
}
