extern crate slack;

use super::SolamiHandler;

pub fn handle(p: &SolamiHandler) {
    if let Ok(resp) = p.send() {
        println!("ok");
    } else {
        println!("fail");
    }
}
