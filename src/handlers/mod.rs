pub mod echo;

pub struct Params<'a> {
    pub user: &'a String,
    pub pattern: &'a str,
    pub channel_id: &'a str,
}
