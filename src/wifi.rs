use crate::ATClient;

pub struct ATWifi<'a> {
    client: &'a mut ATClient,
}

impl<'a> ATWifi<'a> {
    pub fn new(client: &'a mut ATClient) -> Self {
        ATWifi { client }
    }

    pub fn get_current_mode(&mut self) -> String {
        self.client.send("AT+CWMODE_CUR?")
    }

    pub fn set_current_mode(&mut self, mode: WifiMode) -> String {
        let mode = match mode {
            WifiMode::Station => 1,
            WifiMode::SoftAccessPoint => 2,
            WifiMode::SoftAccessPointAndStation => 3,
        };

        let message = format!("AT+CWMODE_CUR={}", mode);
        self.client.send(message.as_str())
    }

    pub fn get_available_access_points(&mut self) -> String {
        self.client.send("AT+CWLAP")
    }

    pub fn connect_to_access_point(&mut self, ssid: &str, password: &str) -> String {
        let message = format!("AT+CWJAP_CUR=\"{}\",\"{}\"", ssid, password);
        self.client.send(message.as_str())
    }

    pub fn get_current_access_point(&mut self) -> String {
        self.client.send("AT+CWJAP_CUR?")
    }

    pub fn disconnect_from_access_point(&mut self) -> String {
        self.client.send("AT+CWQAP")
    }
}

pub enum WifiMode {
    Station,
    SoftAccessPoint,
    SoftAccessPointAndStation,
}
