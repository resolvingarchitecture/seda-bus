[![Build Status](https://travis-ci.com/resolvingarchitecture/seda-bus.svg?branch=master)](https://travis-ci.com/resolvingarchitecture/seda-bus)
# Staged Event-Driven Architecture (SEDA) Bus
A form of message bus avoiding the high overhead of thread-based concurrency models where channels get their own inbound and outbound queues 
each with a thread pool. The bus uses its own thread pool to route messages between queues. Addresses in Routes point to channel names.
When used in conjunction with the [Service Bus](../service-bus), the service bus acts as a layer above the message bus driving
routing based on service operations and/or other logic which results in mappings to the lower message bus address.

[Crates.io](https://crates.io/crates/seda_bus)

!! WIP - not stable until version 1.0 !!

## Design Goals 

* to support something akin to a 'simpler decentralized [Kafka](https://engineering.linkedin.com/kafka/benchmarking-apache-kafka-2-million-writes-second-three-cheap-machines)' placing the burden of queue checks on the clients for maximum scalability
* persist all messages immediately providing configurable retention times
* mark messages as consumed on consumption so that clients can continue on if synchronous
* mark messages as completed on completion so that clients know when they can clear the log if they wish
* mark messages as errored with code on error so that clients can determine how they wish to handle it
* use [dbus](https://en.wikipedia.org/wiki/D-Bus) for inter-process communications on Linux
* use [ipcd](https://dev.to/legolord208/programming-for-redox-os-4124) for inter-process communications on RedoxOS

## Functionality

1. Start message bus up with a name, number of channels, and channel prefix. 
    1. Scheduled Thread Pool is created with name with the number of channels = channels / 2 (+1 if odd number of channels)
    2. Create a Message Channel for each number of channels and name it prefix_number starting with 1.
    3. Allocate an Inbound Persistent Queue and Outbound Persistent Queue per Channel.