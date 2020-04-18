use std::sync::mpsc::{channel, Sender, Receiver};
use std::collections::HashMap;
use std::time::Duration;
use ra_common::models::Envelope;
use std::io::{Error, ErrorKind};

#[derive(Debug, Copy, Clone)]
pub enum BusType {
    Internal,
    DBus,
    IPCD
}

impl BusType {
    pub fn as_string(&self) -> &'static str {
        match *self {
            BusType::Internal => "Internal",
            BusType::DBus => "DBus",
            BusType::IPCD => "IPCD"
        }
    }
    pub fn from_str(sig_type: &str) -> Result<Self, Error> {
        match sig_type {
            "Internal" => Ok(BusType::Internal),
            "DBus" => Ok(BusType::DBus),
            "IPCD" => Ok(BusType::IPCD),
            _ => Result::Err(Error::new(ErrorKind::InvalidData, format!("BusType provided not supported: {}", sig_type)))
        }
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
    b_type: BusType,
    name: String,
    channels: HashMap<u8,MessageChannel>
}

impl MessageBus {
    pub fn new(name: String, b_type: BusType) -> Result<MessageBus,Error> {
        match b_type {
            BusType::Internal => {
                Ok(MessageBus {
                    name,
                    b_type,
                    channels: HashMap::new()
                })
            },
            BusType::DBus => {
                Err(Error::new(ErrorKind::NotFound, "DBus not yet implemented."))
            },
            BusType::IPCD => {
                Err(Error::new(ErrorKind::NotFound, "IPCD not yet implemented."))
            }
        }
    }
    pub fn register(&mut self) -> u8 {
        let id :u8 = self.channels.len() as u8;
        self.channels.insert(id, MessageChannel::new(id));
        id
    }
    pub fn unregister(&mut self, id: u8) -> bool {
        self.channels.remove(&id).is_some()
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