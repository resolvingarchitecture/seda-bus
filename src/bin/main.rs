extern crate log;
extern crate simple_logger;

use log::{trace,info};
use std::thread;
use std::time::Duration;
use seda_bus::{MessageBus, Envelope};

fn main() {
    simple_logger::init().unwrap();
    trace!("Starting SEDA Bus...");

    let mut bus = MessageBus::new();
    let from = bus.register();
    let to = bus.register();

    for n in 1..10 {
        let mut msg = format!("Hello 12: {}",n);
        let env = Envelope::new(from,to, msg.into_bytes());
        bus.send(env);
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
