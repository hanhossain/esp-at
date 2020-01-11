use crate::ATClient;

pub struct ATWifi<'a> {
    client: &'a mut ATClient,
}

impl<'a> ATWifi<'a> {
    pub fn new(client: &'a mut ATClient) -> Self {
        ATWifi { client }
    }

    pub fn get_available_access_points(&mut self) -> String {
        self.client.send("AT+CWLAP")
    }

    pub fn disconnect_from_access_point(&mut self) -> String {
        self.client.send("AT+CWQAP")
    }
}