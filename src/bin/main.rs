extern crate log;
extern crate simple_logger;

use log::{trace,info};
use ra_common::models::{Envelope,LifeCycle,Route};
use seda_bus::bus::MessageBus;
use seda_bus::endpoint::MessageEndpoint;

fn main() {
    simple_logger::init().unwrap();
    trace!("Setting up Message Bus...");
    let mut bus = MessageBus::new(String::from("1M5"));
    let a_id = bus.create_endpoint();
    let mut end_a = bus.endpoint(a_id).unwrap();
    info!("A: {}", end_a.addr());
    let b_id = bus.create_endpoint();
    let mut end_b = bus.endpoint(b_id).unwrap();
    info!("B: {}", end_b.addr());
    let c_id = bus.create_endpoint();
    let mut end_c = bus.endpoint(c_id).unwrap();
    info!("C: {}", end_c.addr());
    let d_id = bus.create_endpoint();
    let mut end_d = bus.endpoint(d_id).unwrap();
    info!("D: {}", end_d.addr());
    bus.start();

    // trace!("Sending test message...");
    // let mut env_in = Envelope::new();
    // env_in.payload = Option::Some(String::from("Hello World"));
    // env_in.slip.add_route(Route::new_msg_route_no_relay(a_id, b_id));
}

#[cfg(test)]
pub mod tests {

    use super::*;
    use crate::bus::{MessageBus, MessageEndpoint};
    use ra_common::models::{Envelope, Route, Producer, Consumer, LifeCycle};

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn bus() {
        // Setup bus and endpoints
        let mut bus = seda::MessageBus::new(String::from("test_bus"));
        let a_id = bus.create_endpoint();
        let mut end_a = bus.endpoint(a_id);
        let b_id = bus.create_endpoint();
        let mut end_b = bus.endpoint(b_id);
        let c_id = bus.create_endpoint();
        let mut end_c = bus.endpoint(c_id);
        let d_id = bus.create_endpoint();
        let mut end_d = bus.endpoint(d_id);
        bus.start();
        assert_eq!(4, bus.num_endpoints());

        // Send envelopes and check
        // let mut env = Envelope::new();
        // let mut r = Route::new_msg_route_no_relay(
        //     String::from("A"),
        //     String::from("B"));
        // env.slip.add_route(r);
        // end_a.send(env);
        // assert message is in B
        // let env = &consumer_b.receive();
        // Route from A to B to C to D using Routes

        // assert message is in D
    }
}