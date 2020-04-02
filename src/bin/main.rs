extern crate log;
extern crate simple_logger;

use log::{trace,info};
use std::thread;
use std::time::Duration;
use seda_bus::{Bus, Envelope};

fn main() {
    simple_logger::init().unwrap();
    trace!("Starting SEDA Bus...");
    let addr :u64 = 12;

    let mut bus = Bus::new();
    bus.register(addr);

    for n in 1..10 {
        let env = Envelope::new(12, format!("Hello World 12: {}",n));
        bus.send(env);
    }

    thread::spawn( move || {
        loop {
            match bus.poll(addr) {
                Some(env) => info!("env to={} msg={}", env.to, env.msg),
                None => info!("x")
            }
        }
    });

    thread::sleep(Duration::from_secs(1));

    trace!("SED Bus Stopped.");
}
