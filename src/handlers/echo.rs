use super::Params;

pub fn handle(p: &Params) {
    println!("{} said {}@{}", p.user, p.pattern, p.channel_id);
}
