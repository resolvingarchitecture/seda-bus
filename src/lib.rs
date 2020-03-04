
mod seda {

    use scheduled_thread_pool::ScheduledThreadPool;

    struct Message {

    }

    struct MessageChannel {

    }

    /// A combination of a common data model, a common command set, and a messaging infrastructure
    /// to allow different systems to communicate through a shared set of interfaces.
    struct MessageBus {
        pool: ScheduledThreadPool
    }

    impl MessageBus {
        fn new() -> MessageBus {
            MessageBus {
                pool: ScheduledThreadPool::with_name( &name, size)
            }
        }
    }

    /// Messaging Client for connecting to a Message Channel as a Producer and/or Consumer
    struct MessageEndpoint {

    }

}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
