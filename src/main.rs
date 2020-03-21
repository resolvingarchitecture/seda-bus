extern crate log;
extern crate simple_logger;

use log::{trace,info,warn};
use std::sync::mpsc::{channel, Sender, Receiver, RecvError, RecvTimeoutError};
use std::collections::HashMap;
use std::thread;
use std::time::Duration;

pub struct Consumer {
    _id: u64
}

impl Consumer {
    pub fn new(id: u64) -> Box<Consumer> {
        Box::new(Consumer {
            _id: id
        })
    }
    pub fn receive(&self, msg: String) {
        info!("Consumer {}: {}",self._id, msg);
    }
}

pub struct MessageChannel {
    pub _accepting: bool,
    pub _addr: u64,
    pub _tx: Sender<String>,
    pub _rx: Receiver<String>
}

impl MessageChannel {
    pub fn new(addr: u64) -> Box<MessageChannel> {
        let (tx, rx) = channel();
        Box::new(MessageChannel {
            _accepting: true,
            _addr: addr,
            _tx: tx,
            _rx: rx
        })
    }
}

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

    for n in 1..10 {
        send_2.send(format!("Hello World 2: {}",n));
    }
    thread::spawn(move || {
        loop {
            let res = rec_1.recv_timeout(Duration::from_millis(100));
            if res.is_ok() {
                c_1.receive(res.unwrap());
            }
        }
    });
    thread::spawn(move || {
        loop {
            let res = rec_2.recv_timeout(Duration::from_millis(100));
            if res.is_ok() {
                c_2.receive(res.unwrap());
            }
        }
    });
    for n in 1..10 {
        send_1.send(format!("Hello World 1: {}",n));
    }
    thread::sleep(Duration::from_secs(1));
    for n in 10..20 {
        send_1.send(format!("Hello World 1: {}",n));
    }
    for n in 10..20 {
        send_2.send(format!("Hello World 2: {}",n));
    }
    thread::sleep(Duration::from_secs(1));
    trace!("SED Bus Stopped.");
}
