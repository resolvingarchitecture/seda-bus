extern crate log;
extern crate simple_logger;

use log::{trace,info,warn};
use std::sync::mpsc::{channel, Sender, Receiver};
use std::collections::HashMap;

pub struct MessageChannel {
    _accepting: bool,
    _tx: Sender<String>,
    _rx: Receiver<String>
}

impl MessageChannel {
    pub fn new() -> Box<MessageChannel> {
        let (tx, rx) = channel();
        Box::new(MessageChannel {
            _accepting: true,
            _tx: tx,
            _rx: rx
        })
    }
    fn send(&mut self, env: String) {
        self._tx.send(env).unwrap();
    }
    fn receive(&mut self) -> String {
        self._rx.recv().unwrap()
    }
}

pub struct MessageBus {
    _ch_1: Box<MessageChannel>,
    _ch_2: Box<MessageChannel>
}

impl MessageBus {
    pub fn new() -> Box<MessageBus> {
        Box::new(MessageBus {
            _ch_1: MessageChannel::new(),
            _ch_2: MessageChannel::new()
        })
    }

    pub fn send(&mut self, to_addr: u64, msg: String) {
        match to_addr {
            1 => self._ch_1.send(msg),
            2 => self._ch_2.send(msg),
            _ => warn!("No channel registered at address {}",to_addr)
        }
    }

    pub fn recv(&mut self, from_addr: u64) -> String {
        match from_addr {
            1 => self._ch_1.receive(),
            2 => self._ch_2.receive(),
            _ => String::from("None")
        }
    }
}

fn main() {
    simple_logger::init().unwrap();
    trace!("Starting SEDA Bus...");
    let mut bus = MessageBus::new();
    bus.send(1, String::from("Hello World"));
    let msg = bus.recv(1);
    info!("{}",msg);
    trace!("SED Bus Stopped.");
}
