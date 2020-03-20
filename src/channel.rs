use std::sync::mpsc::{channel, Sender, Receiver, RecvError, SendError};
use ra_common::models::{Consumer,Envelope,Producer};
/// A Channel with Sender and Receiver
#[derive(Debug)]
pub struct MessageChannel {
    _accepting: bool,
    _tx: Sender<Box<Envelope>>,
    _rx: Receiver<Box<Envelope>>
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
}

impl Producer for MessageChannel {
    fn send(&mut self, env: Box<Envelope>) {
        if self._accepting {
            // loop {
            //     info!("{}","Sending...");
            // wait::wait_a_ms(1000);
            // }
            self._tx.send(env).unwrap();
        }
    }
}

impl Consumer for MessageChannel {
    fn receive(&mut self) -> Box<Envelope> {
        self._rx.recv().unwrap()
    }
}