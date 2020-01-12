use crate::ATClient;

pub struct ATBasic<'a> {
    client: &'a mut ATClient,
}

impl<'a> ATBasic<'a> {
    pub fn new(client: &'a mut ATClient) -> Self {
        ATBasic { client }
    }

    pub fn test_startup(&mut self) -> String {
        self.client.send("AT")
    }

    pub fn get_status(&mut self) -> String {
        self.client.send("AT+GMR")
    }
}
