extern crate log;
extern crate simple_logger;

use log::{trace,info,warn};
use std::sync::mpsc::{channel, Sender, Receiver, RecvError, RecvTimeoutError};
use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use seda_bus::{MessageChannel, Router, Envelope};


fn main() {
    simple_logger::init().unwrap();
    trace!("Starting SEDA Bus...");
    let addr :u64 = 12;

    let mut r = Router::new();
    r.register(addr);

    for n in 1..10 {
        let env = Envelope::new(12, format!("Hello World 12: {}",n));
        r.send(env);
    }

    thread::spawn( move || {
        loop {
            match r.poll(addr) {
                Some(env) => info!("env to={} msg={}", env.to, env.msg),
                None => info!("x")
            }
        }
    });

    thread::sleep(Duration::from_secs(1));

    trace!("SED Bus Stopped.");
}
