use std::sync::mpsc::{channel, Sender, Receiver};
use std::collections::HashMap;
use std::time::Duration;

/// An Envelope is a wrapper of data with some meta-data for internal routing.
pub struct Envelope {
    pub from: u8,
    pub to: u8,
    pub msg: Vec<u8>
}

impl Envelope {
    pub fn new(from: u8, to: u8, msg: Vec<u8>) -> Envelope {
        Envelope { from, to, msg }
    }
}

pub struct MessageChannel {
    pub addr: u8,
    pub tx: Sender<Envelope>,
    pub rx: Receiver<Envelope>
}

impl MessageChannel {
    pub fn new(addr: u8) -> MessageChannel {
        let (tx, rx) = channel();
        MessageChannel { addr, tx, rx }
    }
}

pub struct MessageBus {
    channels: HashMap<u8,MessageChannel>
}

impl MessageBus {
    pub fn new() -> MessageBus {
        MessageBus {
            channels: HashMap::new()
        }
    }
    pub fn register(&mut self) -> u8 {
        let id :u8 = self.channels.len() as u8;
        self.channels.insert(id, MessageChannel::new(id));
        id
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
    pub fn poll(&mut self, addr: u8) -> Option<Envelope> {
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
    pub fn poll_wait(&mut self, addr: u8, wait: u64) -> Option<Envelope> {
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