
/// A combination of a common data model, a common command set, and a messaging infrastructure
/// to allow different systems to communicate through a shared set of interfaces.
///
/// The MessageBus provides a means to create MessageEndpoints by providing an address and
/// a Consumer returning a Producer the client can use to send Envelopes.
pub mod bus {

    use log::{info};
    use std::collections::HashMap;
    use std::sync::mpsc::{channel, Sender, Receiver, RecvError, SendError};
    use std::thread;
    use rand::{Rng, RngCore};

    use ra_common::{Envelope, Route, LifeCycle, Consumer, Producer, Router};

    const MAXIMUM_CAPACITY: usize = 10;

    // pub fn run() {
    //     let running = false;
    //     let endpoints = HashMap::with_capacity(MAXIMUM_CAPACITY);
    //
    //
    //     info!("Called run.");
    // }

    /// A Channel with Sender and Receiver
    #[derive(Debug)]
    struct MessageChannel {
        _accepting: bool,
        _tx: Sender<Box<Envelope>>,
        _rx: Receiver<Box<Envelope>>
    }

    impl MessageChannel {
        fn new() -> Box<MessageChannel> {
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
                self._tx.send(env).unwrap();
            }
        }
    }

    impl Consumer for MessageChannel {
        fn receive(&mut self) -> Box<Envelope> {
            self._rx.recv().unwrap()
        }
    }

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

        fn send(&mut self, env: Box<Envelope>) {
            self._in.send(env);
        }

        fn receive(&mut self) -> Box<Envelope> {
            self._out.receive()
        }
    }

    impl Producer for MessageEndpoint {
        fn send(&mut self, env: Box<Envelope>) {
            self._out.send(env);
        }
    }

    impl Consumer for MessageEndpoint {
        fn receive(&mut self) -> Box<Envelope> {
            self._in.receive()
        }
    }

    pub struct MessageBus {
        _running: bool,
        _endpoints: HashMap<u64, Box<MessageEndpoint>>
    }

    impl MessageBus {
        pub fn new(name: String) -> Box<MessageBus> {
            Box::new(MessageBus {
                _running: false,
                _endpoints: HashMap::with_capacity(MAXIMUM_CAPACITY)
            })
        }

        pub fn create_endpoint(&mut self) -> u64 {
            let addr = rand::thread_rng().next_u64();
            self._endpoints.insert(addr, MessageEndpoint::new(addr));
            return addr;
        }

        pub fn endpoint(&mut self, addr: u64) -> &Box<MessageEndpoint> {
            self._endpoints.get(&addr).unwrap()
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
            // Use current thread with asynch support to loop through each endpoint checking for messages in its _out channel
            // 1. Consume on each Endpoint's _out channel
            // 2. Route when envelopes shows up grabbing channel based on the slip route's destination address
            // 3. Producing on each _in channel
            while self._running {
                let endpoints = &mut self._endpoints;
                for addr in endpoints.keys() {
                    let ep = &mut endpoints.get(addr).unwrap();
                    info!("endpoint: {:#?}", ep);
                    // TODO: Unable to call receive
                    // ep.receive();
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
}

#[cfg(test)]
pub mod tests {

    use super::*;
    use crate::seda::{MessageBus, MessageEndpoint};
    use ra_common::{LogConsumer, Envelope, Route, Producer, Consumer, LifeCycle};

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn bus() {
        // Setup bus and endpoints
        let mut bus = seda::MessageBus::new(String::from("test_bus"));
        let a_id = bus.create_endpoint();
        let mut end_a = bus.endpoint(a_id);
        let b_id = bus.create_endpoint();
        let mut end_b = bus.endpoint(b_id);
        let c_id = bus.create_endpoint();
        let mut end_c = bus.endpoint(c_id);
        let d_id = bus.create_endpoint();
        let mut end_d = bus.endpoint(d_id);
        bus.start();
        assert_eq!(4, bus.num_endpoints());

        // Send envelopes and check
        // let mut env = Envelope::new();
        // let mut r = Route::new_msg_route_no_relay(
        //     String::from("A"),
        //     String::from("B"));
        // env.slip.add_route(r);
        // end_a.send(env);
        // assert message is in B
        // let env = &consumer_b.receive();
        // Route from A to B to C to D using Routes

        // assert message is in D
    }
}
