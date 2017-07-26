extern crate slack;

use std::collections::HashMap;
use slack::{Sender, User};

pub mod inc;
pub mod echo;
pub mod rss;

pub struct SolamiHandler<'a> {
    pub sender: &'a Sender,
    pub channel_id: &'a str,
}

pub type Users = HashMap<String, User>;

impl<'a> SolamiHandler<'a> {
    pub fn send(&self, body: &'a str) -> Result<usize, slack::error::Error> {
        self.sender.send_message(self.channel_id, body)
    }
}
