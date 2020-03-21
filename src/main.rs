extern crate log;
extern crate simple_logger;

use log::{trace,info,warn};
use std::sync::mpsc::{channel, Sender, Receiver, RecvError, RecvTimeoutError};
use std::collections::HashMap;
use std::thread;
use std::time::Duration;

pub struct MessageChannel {
    pub _accepting: bool,
    pub _tx: Sender<String>,
    pub _rx: Receiver<String>
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
    fn receive(&mut self) -> Result<String,RecvTimeoutError> {
        info!("{}",".");
        self._rx.recv_timeout(Duration::from_millis(1000))
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

    pub fn recv(&mut self, from_addr: u64) -> Result<String,RecvTimeoutError> {
        match from_addr {
            1 => self._ch_1.receive(),
            2 => self._ch_2.receive(),
            _ => Ok(String::from("None"))
        }
    }
}

fn main() {
    simple_logger::init().unwrap();
    trace!("Starting SEDA Bus...");
    let mut bus = MessageBus::new();
    let mut rec_1 = bus._ch_1._rx;
    let mut send_1 = bus._ch_1._tx;
    thread::spawn(move || {
        loop {
            let res = rec_1.recv_timeout(Duration::from_millis(1000));
            if res.is_ok() {
                info!("{}", res.unwrap());
            } else {
                info!("{}", ".");
            }
        }
    });
    // for n in 1..10 {
    //     bus.send(2, format!("Hello World 2: {}",n));
    // }
    for n in 1..20 {
        send_1.send(format!("Hello World 1: {}",n));
    }
    thread::sleep(Duration::from_secs(5));
    for n in 21..100 {
        send_1.send(format!("Hello World 1: {}",n));
    }
    thread::sleep(Duration::from_secs(5));
    trace!("SED Bus Stopped.");
}
