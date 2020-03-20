use log::{info};
use std::vec::Vec;
use std::thread;
use rand::{Rng, RngCore};
use crate::endpoint::MessageEndpoint;
use ra_common::models::{Consumer,Envelope,LifeCycle,Producer,Route};

const MAXIMUM_CAPACITY: usize = 10;

/// The MessageBus provides a means to create MessageEndpoints by providing an address and
/// a Consumer returning a Producer the client can use to send Envelopes.
pub struct MessageBus {
    _running: bool,
    _endpoints: Vec<Box<MessageEndpoint>>
}

impl MessageBus {
    pub fn new(name: String) -> Box<MessageBus> {
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

    pub fn endpoint(&mut self, addr: u64) -> Option<&Box<MessageEndpoint>> {
        for endpoint in &self._endpoints {
            if endpoint.addr().eq(&addr) {
                return Option::Some(endpoint);
            }
        }
        return None;
    }

    pub fn num_endpoints(&mut self) -> usize {
        self._endpoints.len()
    }

}

impl LifeCycle for MessageBus {
    fn start(&mut self) {
        info!("{}","SEDA MessageBus starting...");
        info!("{} Endpoints", &self._endpoints.len());
        self._running = true;
        info!("{}","SEDA MessageBus running...");
        while self._running {
            let endpoints = &mut self._endpoints;
            for end_in in endpoints {
                thread::spawn(move || {
                    while self._running {
                        info!("receiving on endpoint: {:#?}", end_in);
                        let env_out = end_in.receive();
                        info!("received envelope: {:#?}", env_out);
                        let route = env_out.slip.current_route().unwrap();
                        let end_out = &mut self.endpoint(route._dest).unwrap();
                        end_out.send(env_out);
                    }
                });
            }
            self._running = false;
        }
        info!("{}","SEDA MessageBus stopped");
    }

    fn restart(&mut self) {
        unimplemented!()
    }

    fn pause(&mut self) {
        unimplemented!()
    }

    fn unpause(&mut self) {
        unimplemented!()
    }

    fn stop(&mut self) {
        self._running = false;
    }

    fn graceful_stop(&mut self) {
        self._running = false;
    }
}