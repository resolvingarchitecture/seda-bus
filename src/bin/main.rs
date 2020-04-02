extern crate log;
extern crate simple_logger;

use log::{trace,info,warn};
use std::sync::mpsc::{channel, Sender, Receiver, RecvError, RecvTimeoutError};
use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use seda_bus::{Consumer, MessageChannel, Router, Envelope};


fn main() {
    simple_logger::init().unwrap();
    trace!("Starting SEDA Bus...");

    let mut c_1 = Consumer::new(1);
    let mut ch_1 = MessageChannel::new(1);
    let mut rec_1 = ch_1._rx;
    let mut send_1 = ch_1._tx;

    let mut c_2 = Consumer::new(2);
    let mut ch_2 = MessageChannel::new(2);
    let mut rec_2 = ch_2._rx;
    let mut send_2 = ch_2._tx;

    let mut r = Router::new(send_1, send_2);

    for n in 1..10 {
        let env = Envelope::new(2, format!("Hello World 2: {}",n));
        r.route(env);
    }
    for n in 1..10 {
        let env = Envelope::new(1, format!("Hello World 1: {}",n));
        r.route(env);
    }
    thread::spawn(move || {
        loop {
            match rec_1.recv_timeout(Duration::from_millis(100)) {
                Ok(env) => c_1.receive(env),
                Err(e) => info!("{}","timedout")
            }
        }
    });
    thread::spawn(move || {
        loop {
            match rec_2.recv_timeout(Duration::from_millis(100)) {
                Ok(env) => c_2.receive(env),
                Err(e) => info!("{}","timedout")
            }
        }
    });
    for n in 10..20 {
        let env = Envelope::new(1, format!("Hello World 1: {}",n));
        r.route(env);
    }
    thread::sleep(Duration::from_secs(1));
    for n in 10..20 {
        let env = Envelope::new(2, format!("Hello World 2: {}",n));
        r.route(env);
    }

    thread::sleep(Duration::from_secs(1));
    trace!("SED Bus Stopped.");
}
