[![Build Status](https://travis-ci.com/resolvingarchitecture/seda-bus.svg?branch=master)](https://travis-ci.com/resolvingarchitecture/seda-bus)
# Staged Event-Driven Architecture (SEDA) Bus
A form of message bus avoiding the high overhead of thread-based concurrency models where channels get their own inbound and outbound queues. 
When used in conjunction with the [Service Bus](https://github.com/resolvingarchitecture/service-bus), 
the service bus acts as a layer above the message bus driving routing based on service operations and/or other 
logic which results in mappings to the lower message bus address.

[Crates.io](https://crates.io/crates/seda_bus)

!! WIP - not stable until version 1.0 !!

## Design Goals 

*[x] non-persistence of messages for deniability by default
*[ ] mark messages as consumed on consumption so that clients can continue on if synchronous
*[ ] mark messages as completed on completion so that clients know when they can clear the queue if they wish
*[ ] mark messages as errored with code on error so that clients can determine how they wish to handle it
*[ ] use [dbus](https://en.wikipedia.org/wiki/D-Bus) for inter-process communications on Linux
*[ ] use [ipcd](https://dev.to/legolord208/programming-for-redox-os-4124) for inter-process communications on RedoxOS

## Functionality

### lib.rs
1. Start Bus
2. Register Endpoints as needed using Bus receiving Id
3. Send Envelopes
4. Poll with Ids

### main.rs
1. cargo install seda_bus
2. copy seda_bus file to: /etc/init.d/seda_bus
3. chmod +x /etc/init.d/seda_bus -v
4. service seda_bus start

   