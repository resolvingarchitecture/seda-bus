use ra_common::models::{Consumer,Envelope,Producer};
use crate::channel::MessageChannel;
/// An address on the network with its inbound and outbound channels.
#[derive(Debug)]
pub struct MessageEndpoint {
    _address: u64,
    _in: Box<MessageChannel>,
    _out: Box<MessageChannel>
}

impl MessageEndpoint {
    pub fn new(address: u64) -> Box<MessageEndpoint> {
        Box::new(MessageEndpoint {
            _address: address,
            _in: MessageChannel::new(),
            _out: MessageChannel::new()
        })
    }

    pub fn addr(&self) -> u64 {
        self._address
    }

    pub fn receive(&mut self) -> Box<Envelope> {
        self._out.receive()
    }

    pub fn send(&mut self, env: Box<Envelope>) {
        self._out.send(env);
    }
}

impl Producer for MessageEndpoint {
    fn send(&mut self, env: Box<Envelope>) {
        self._out.send(env);
    }
}

impl Consumer for MessageEndpoint {
    fn receive(&mut self) -> Box<Envelope> {
        self._out.receive()
    }
}