extern crate log;

use log::{trace,info,warn};
use std::sync::mpsc::{channel, Sender, Receiver, RecvError, RecvTimeoutError, SendError};
use std::collections::HashMap;
use std::thread;
use std::time::Duration;

#[derive(Clone)]
pub struct Envelope {
    pub to: u64,
    pub msg: String
}

impl Envelope {
    pub fn new(to: u64, msg: String) -> Envelope {
        Envelope { to, msg }
    }
}

pub struct MessageChannel {
    pub accepting: bool,
    pub addr: u64,
    pub tx: Sender<Envelope>,
    pub rx: Receiver<Envelope>
}

impl MessageChannel {
    pub fn new(addr: u64) -> MessageChannel {
        let (tx, rx) = channel();
        MessageChannel { accepting: true, addr, tx, rx }
    }
}

pub struct Router {
    channels: HashMap<u64,MessageChannel>
}

impl Router {
    pub fn new() -> Router {
        Router {
            channels: HashMap::new()
        }
    }
    pub fn register(&mut self, addr: u64) {
        self.channels.insert(addr, MessageChannel::new(addr));
    }
    pub fn register_all(&mut self, addr: [u64;2]) {
        info!("register {:?}",addr);
        for i in 0..1 {
            self.channels.insert(addr[i], MessageChannel::new(addr[i]));
        }
    }
    pub fn poll(&mut self, addr: u64) -> Option<Envelope> {
        match self.channels.get(&addr) {
            Some(ch) => {
                match ch.rx.recv_timeout(Duration::from_millis(100)) {
                    Ok(env) => Option::Some(env),
                    Err(e) => Option::None
                }
            },
            None => Option::None
        }
    }
    pub fn route(&mut self, env: Envelope) -> bool {
        match self.channels.get(&env.to) {
            Some(ch) => {
                match ch.tx.send(env) {
                    Ok(()) => true,
                    Err(e) => false
                }
            },
            None => false
        }
    }
}