use esp_at::ATClient;
use std::time::Duration;

use serial::prelude::*;
use esp_at::basic::ATBasic;
use esp_at::wifi::ATWifi;

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
    let mut at_basic = ATBasic::new(&mut at_client);

    println!("checking startup...");
    println!("{}", at_basic.test_startup());

    println!("getting status...");
    println!("{}", at_basic.get_status());

    let mut at_wifi = ATWifi::new(&mut at_client);
    println!("getting access points...");
    println!("{}", at_wifi.get_available_access_points());

    println!("disconnecting from  access point...");
    println!("{}", at_wifi.disconnect_from_access_point());
}
