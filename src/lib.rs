extern crate log;

use log::{info};
use std::sync::mpsc::{channel, Sender, Receiver};
use std::collections::HashMap;
use std::time::Duration;

pub struct Envelope {
    pub from: u64,
    pub to: u64,
    pub msg: String
}

impl Envelope {
    pub fn new(from: u64, to: u64, msg: String) -> Envelope {
        Envelope { from, to, msg }
    }
}

pub struct MessageChannel {
    pub addr: u64,
    pub tx: Sender<Envelope>,
    pub rx: Receiver<Envelope>
}

impl MessageChannel {
    pub fn new(addr: u64) -> MessageChannel {
        let (tx, rx) = channel();
        MessageChannel { addr, tx, rx }
    }
}

pub struct MessageBus {
    channels: HashMap<u64,MessageChannel>
}

impl MessageBus {
    pub fn new() -> MessageBus {
        MessageBus {
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
    pub fn send(&mut self, env: Envelope) -> bool {
        match self.channels.get(&env.to) {
            Some(ch) => {
                match ch.tx.send(env) {
                    Ok(()) => true,
                    Err(_) => false
                }
            },
            None => false
        }
    }
    pub fn poll(&mut self, addr: u64) -> Option<Envelope> {
        match self.channels.get(&addr) {
            Some(ch) => {
                match ch.rx.try_recv() {
                    Ok(env) => Option::Some(env),
                    Err(_) => Option::None
                }
            },
            None => Option::None
        }
    }
    pub fn poll_wait(&mut self, addr: u64, wait: u64) -> Option<Envelope> {
        match self.channels.get(&addr) {
            Some(ch) => {
                match ch.rx.recv_timeout(Duration::from_millis(wait)) {
                    Ok(env) => Option::Some(env),
                    Err(_) => Option::None
                }
            },
            None => Option::None
        }
    }
}