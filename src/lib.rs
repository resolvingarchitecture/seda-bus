
mod seda {

    use scheduled_thread_pool::ScheduledThreadPool;

    struct MessageChannel {

    }

    /// A combination of a common data model, a common command set, and a messaging infrastructure
    /// to allow different systems to communicate through a shared set of interfaces.
    /// This MessageBus takes
    struct MessageBus {
        pool: ScheduledThreadPool
    }

    impl MessageBus {
        fn new(name: &mut String, num_channels: usize) -> MessageBus {
            let mut num_threads: i8 = (num_channels / 2) as i8;
            if num_threads % 2 != 0 {
                num_threads += 1; // add one for dead-letter channel
            }
            MessageBus {
                pool: ScheduledThreadPool::with_name( &name, num_channels)
            }
        }
    }

}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn bus() {

    }
}
