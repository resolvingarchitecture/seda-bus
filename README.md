[![Build Status](https://travis-ci.com/resolvingarchitecture/seda-bus.svg?branch=master)](https://travis-ci.com/resolvingarchitecture/seda-bus)
# Staged Event-Driven Architecture (SEDA) Bus
A form of message bus where channels get their own queue and a thread pool manages
communications between the queues using their own worker threads to avoid the high
overhead of with thread-based concurrency models.

[Crates.io](https://crates.io/crates/seda_bus)

!! WIP - not stable until version 1.0 !!