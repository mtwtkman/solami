extern crate slack;

pub mod echo;

pub struct SolamiHandler<'a> {
    pub sender: &'a slack::Sender,
    pub pattern: &'a str,
    pub channel_id: &'a str,
}

impl<'a> SolamiHandler<'a> {
    fn send(&self) -> Result<usize, slack::error::Error> {
        self.sender.send_message(self.channel_id, self.pattern)
    }
}
