use log::{info,warn};
use std::vec::Vec;
use rand::{RngCore};
use futures::{executor, Future};

use crate::endpoint::MessageEndpoint;
use ra_common::models::{Consumer,Envelope,Producer,Route};

const MAXIMUM_CAPACITY: usize = 10;

/// The MessageBus provides a means to create MessageEndpoints by providing an address and
/// a Consumer returning a Producer the client can use to send Envelopes.
pub struct MessageBus {
    _running: bool,
    _endpoints: Vec<Box<MessageEndpoint>>
}

impl MessageBus {
    pub fn new() -> Box<MessageBus> {
        Box::new(MessageBus {
            _running: false,
            _endpoints: Vec::with_capacity(MAXIMUM_CAPACITY)
        })
    }

    pub fn create_endpoint(&mut self) -> u64 {
        let addr = rand::thread_rng().next_u64();
        self._endpoints.push(MessageEndpoint::new(addr));
        return addr;
    }

    pub fn endpoint(&mut self, addr: u64) -> Option<&mut Box<MessageEndpoint>> {
        for endpoint in &mut self._endpoints {
            if endpoint.addr().eq(&addr) {
                return Some(endpoint);
            }
        }
        return None;
    }

    pub fn num_endpoints(&mut self) -> usize {
        self._endpoints.len()
    }

}