use esp_at::wifi::ATWifi;
use esp_at::ATClient;
use serial::prelude::*;
use std::time::Duration;

fn main() {
    let mut port = serial::open("/dev/cu.usbserial-AL00WS14").unwrap();

    port.reconfigure(&|settings| {
        settings.set_baud_rate(serial::Baud115200)?;
        settings.set_char_size(serial::Bits8);
        settings.set_parity(serial::ParityNone);
        settings.set_stop_bits(serial::Stop1);
        settings.set_flow_control(serial::FlowNone);
        Ok(())
    })
    .unwrap();

    port.set_timeout(Duration::from_millis(30_000)).unwrap();

    let mut at_client = ATClient::new(port);
    let mut at_wifi = ATWifi::new(&mut at_client);

    println!("Current AP");
    println!("{}", at_wifi.get_current_access_point());

    println!("Connecting...");
    let ssid = "ssid";
    let password = "password";
    println!("{}", at_wifi.connect_to_access_point(ssid, password));

    println!("Current AP");
    println!("{}", at_wifi.get_current_access_point());

    println!("Disconnecting...");
    println!("{}", at_wifi.disconnect_from_access_point());

    println!("Current AP");
    println!("{}", at_wifi.get_current_access_point());
}
