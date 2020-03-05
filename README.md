[![Build Status](https://travis-ci.com/resolvingarchitecture/seda-bus.svg?branch=master)](https://travis-ci.com/resolvingarchitecture/seda-bus)
# Staged Event-Driven Architecture (SEDA) Bus
A form of message bus avoiding the high overhead of thread-based concurrency models where channels get their own inbound and outbound queues 
each with a thread pool. The bus uses its own thread pool to route messages between queues. Addresses in Routes point to channel names.
When used in conjunction with the [Service Bus](../service-bus), the service bus acts as a layer above the message bus driving
routing based on service operations and/or other logic which results in mappings to the lower message bus address.

[Crates.io](https://crates.io/crates/seda_bus)

!! WIP - not stable until version 1.0 !!

## Design Goals 

* use [ipcd](https://dev.to/legolord208/programming-for-redox-os-4124) for inter-process communications on RedoxOS
* use [dbus](https://en.wikipedia.org/wiki/D-Bus) for inter-process communications on Linux
* to support something akin to a 'simpler decentralized [Kafka](https://engineering.linkedin.com/kafka/benchmarking-apache-kafka-2-million-writes-second-three-cheap-machines)' placing the burden of queue checks on the clients for maximum scalability
* persist all messages immediately providing configurable retention times
* mark messages as consumed on consumption so that clients can continue on if synchronous
* eventually supporting the ability to replicate logs encrypted to other machines for backup

## Functionality

