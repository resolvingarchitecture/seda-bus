extern crate log;

use log::{trace,info,warn};
use std::sync::mpsc::{channel, Sender, Receiver, RecvError, RecvTimeoutError};
use std::collections::HashMap;
use std::thread;
use std::time::Duration;

pub struct Envelope {
    _to_addr: u64,
    _msg: String
}

impl Envelope {
    pub fn new(to_addr: u64, msg: String) -> Envelope {
        Envelope {
            _to_addr: to_addr,
            _msg: msg
        }
    }
}

pub struct Consumer {
    _id: u64
}

impl Consumer {
    pub fn new(id: u64) -> Consumer {
        Consumer {
            _id: id
        }
    }
    pub fn receive(&self, env: Envelope) {
        info!("Consumer {}: {}",self._id, env._msg);
    }
}

pub struct MessageChannel {
    pub _accepting: bool,
    pub _addr: u64,
    pub _tx: Sender<Envelope>,
    pub _rx: Receiver<Envelope>
}

impl MessageChannel {
    pub fn new(addr: u64) -> MessageChannel {
        let (tx, rx) = channel();
        MessageChannel {
            _accepting: true,
            _addr: addr,
            _tx: tx,
            _rx: rx
        }
    }
}

pub struct Router {
    _send_1: Sender<Envelope>,
    _send_2: Sender<Envelope>
}

impl Router {
    pub fn new(send_1: Sender<Envelope>, send_2: Sender<Envelope>) -> Router {
        Router {
            _send_1: send_1,
            _send_2: send_2
        }
    }
    pub fn route(&mut self, env: Envelope) {
        if env._to_addr == 1 {
            self._send_1.send(env);
        } else if env._to_addr == 2 {
            self._send_2.send(env);
        } else {
            warn!("address {} has no channel",env._to_addr);
        }
    }
}