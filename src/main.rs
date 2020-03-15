extern crate log;
extern crate simple_logger;

// mod seda;

use log::{trace};
use ra_common::LifeCycle;

fn main() {
    simple_logger::init().unwrap();
    trace!("Starting SEDA Bus Daemon...");
    // let mut bus = seda::bus::MessageBus::new(String::from("SEDA"));
    // let a_id = bus.create_endpoint();
    // let mut end_a = bus.endpoint(a_id);
    // let b_id = bus.create_endpoint();
    // let mut end_b = bus.endpoint(b_id);
    // let c_id = bus.create_endpoint();
    // let mut end_c = bus.endpoint(c_id);
    // let d_id = bus.create_endpoint();
    // let mut end_d = bus.endpoint(d_id);
    // bus.start();
    // seda::bus::run();
    trace!("SEDA Bus Daemon Stopped.");
}