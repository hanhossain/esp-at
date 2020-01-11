use crate::ATClient;

pub struct ATBasic {
    client: ATClient
}

impl ATBasic {
    pub fn new(client: ATClient) -> Self {
        ATBasic { client }
    }

    pub fn test_startup(&mut self) -> String {
        self.client.send("AT")
    }

    pub fn get_status(&mut self) -> String {
        self.client.send("AT+GMR")
    }

    pub fn list_available_aps(&mut self) -> String {
        self.client.send("AT+CWLAP")
    }
}
