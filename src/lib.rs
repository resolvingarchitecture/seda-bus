
/// A combination of a common data model, a common command set, and a messaging infrastructure
/// to allow different systems to communicate through a shared set of interfaces.
/// This MessageBus takes
mod seda {

    use scheduled_thread_pool::ScheduledThreadPool;
    use std::collections::HashMap;
    use std::sync::mpsc::{channel, Sender, Receiver, RecvError, SendError};
    use std::thread;

    use ra_common::{Envelope, Route, LifeCycle, Consumer, Producer, Router};

    const MAXIMUM_CAPACITY: usize = 10;

    /// An address on the network with its inbound and outbound channels.
    struct MessageEndpoint {
        _address: String,
        _in: Box<MessageChannel>,
        _out: Box<MessageChannel>
    }

    impl MessageEndpoint {
        fn new(address: String, consumer: Box<dyn Consumer>, bus: Box<dyn Consumer>) -> Box<MessageEndpoint> {
            Box::new(MessageEndpoint {
                _address: address,
                _in: MessageChannel::new(consumer),
                _out: MessageChannel::new(bus)
            })
        }
    }

    impl Producer for MessageEndpoint {
        fn send(&self, env: Box<Envelope>) -> Result<(), SendError<Box<Envelope>>> {
            self._out._sender.send(env)
        }
    }

    struct MessageChannel {
        _accepting: bool,
        _sender: Sender<Box<Envelope>>,
        _receiver: Receiver<Box<Envelope>>,
        _consumer: Box<dyn Consumer>
    }

    impl MessageChannel {
        fn new(consumer: Box<dyn Consumer>) -> Box<MessageChannel> {
            let (sender, receiver) = channel();
            Box::new(MessageChannel {
                _accepting: false,
                _sender: sender,
                _receiver: receiver,
                _consumer: consumer
            })
        }
    }

    pub struct MessageBus {
        _pool: ScheduledThreadPool,
        _endpoints: HashMap<String, Box<MessageEndpoint>>
    }

    impl LifeCycle for MessageBus {
        fn start(&self) {
            unimplemented!()
        }

        fn restart(&self) {
            unimplemented!()
        }

        fn pause(&self) {
            unimplemented!()
        }

        fn unpause(&self) {
            unimplemented!()
        }

        fn stop(&self) {
            unimplemented!()
        }

        fn graceful_stop(&self) {
            unimplemented!()
        }
    }

    impl Router for MessageBus {
        fn route(&self, env: Box<Envelope>) -> Option<Route> {
            unimplemented!()
        }
    }

    impl Consumer for MessageBus {
        fn receive(&self, env: Box<Envelope>) {
            let opt = self.route(env);
        }
    }

    impl MessageBus {
        pub fn new(name: String) -> Box<MessageBus> {
            let endpoints = HashMap::with_capacity(MAXIMUM_CAPACITY);
            Box::new(MessageBus {
                _pool: ScheduledThreadPool::with_name( &name, MAXIMUM_CAPACITY),
                _endpoints: endpoints
            })
        }
        // TODO: Implement create_endpoint returning Producer trait with underlying MessageBus
        // pub fn create_endpoint(&mut self, address: String, consumer: Box<dyn Consumer>) -> &Box<dyn Producer> {
        //     let mut endpoint = MessageEndpoint::new(address, consumer, Box::new(self));
        //     self._endpoints.insert(&address, endpoint);
        //     return &endpoint;
        // }

    }

}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::seda::MessageBus;
    use ra_common::{LogConsumer, Envelope, Route, Producer, Consumer};

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn bus() {
        // Setup bus and endpoints
        let mut bus = MessageBus::new(String::from("test_bus"));
        let consumer_a = LogConsumer::new();
        let end_a = bus.create_endpoint(String::from("A"), consumer_a);
        let consumer_b = LogConsumer::new();
        let end_b = bus.create_endpoint(String::from("B"), consumer_b);
        let consumer_c = LogConsumer::new();
        let end_c = bus.create_endpoint(String::from("C"), consumer_c);
        let consumer_d = LogConsumer::new();
        let end_d = bus.create_endpoint(String::from("D"), consumer_d);
        // Send envelopes and check
        let env = Envelope::new();
        let r = Route::new(
            String::from("ServiceA"),
            String::from("send_to_peer"),
            String::from("A"),
            String::from("B"),
            String::from("A"),
            String::from("B"));
        env.slip.add_route(r);
        end_a.send(env);
        // assert message is in B

        // Route from A to B to C to D using Routes

        // assert message is in D
    }
}
