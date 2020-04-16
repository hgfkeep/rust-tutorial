#[macro_use] extern crate log;
#[macro_use] extern crate pretty_env_logger;

use log::Level;

fn main() {
    pretty_env_logger::init();

    debug!("this is a debug");
    error!("this is a error");

    if log_enabled!(Level::Info){
        let x = 3*4;
        info!("this answer is {}", x);
    }
}
