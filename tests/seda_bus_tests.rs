extern crate log;
extern crate simple_logger;

use std::thread;
use log::{trace,info};

use ra_common::models::{Envelope,LifeCycle,Route};
use seda_bus::bus::MessageBus;
use seda_bus::endpoint::MessageEndpoint;

#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}
#[test]
fn test_bus() {
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
    thread::spawn(move || &mut bus.start());
    let mut env_in = Envelope::new();
    env_in.payload = Option::Some(String::from("Hello World"));
    env_in.slip.add_route(Route::new_msg_route_no_relay(ep.addr(), ep.addr()));
    info!("sending envelope to endpoint {:#?}", ep);
    ep.send(env_in);

}