extern crate log;
extern crate simple_logger;

use log::{trace,info};
use std::{thread, env};
use std::time::Duration;
use seda_bus::{MessageBus, BusType};
use ra_common::models::Envelope;

fn main() {
    let args: Vec<String> = env::args().collect();
    let bus_name = &args[1];
    let bus_type = &args[2];
    println!("name: {}, type: {}", bus_name, bus_type);
    simple_logger::init().unwrap();
    trace!("Starting RA SEDA Bus...");

    let mut bus = MessageBus::new(String::from(bus_name), BusType::from_str(bus_type).unwrap());
    let from = bus.register();
    let to = bus.register();

    for n in 1..10 {
        bus.send(Envelope::new(from,to, format!("Hello 12: {}",n).into_bytes()));
    }

    thread::spawn( move || {
        loop {
            match bus.poll_wait(to, 100) {
                Some(env) => info!("env from={} to={} msg={}", env.from, env.to, String::from_utf8(env.msg).unwrap()),
                None => info!("x")
            }
        }
    });

    thread::sleep(Duration::from_secs(1));

    trace!("SED Bus Stopped.");
}
