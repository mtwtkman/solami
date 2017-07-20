extern crate slack;

use std::collections::HashMap;
use slack::{Sender, User};
pub mod yamabiko;
pub mod inc;

pub struct SolamiHandler<'a> {
    pub sender: &'a Sender,
    pub body: &'a str,
    pub channel_id: &'a str,
}

pub type Users = HashMap<String, User>;

impl<'a> SolamiHandler<'a> {
    fn send(&self) -> Result<usize, slack::error::Error> {
        self.sender.send_message(self.channel_id, self.body)
    }

    fn send_with_body(&self, body: &'a str) -> Result<usize, slack::error::Error> {
        self.sender.send_message(self.channel_id, body)
    }
}
