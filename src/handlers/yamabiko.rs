extern crate slack;

use super::SolamiHandler;

pub fn handle(p: SolamiHandler) {
    if let Ok(_) = p.send() {
        println!("ok");
    } else {
        println!("fail");
    }
}
