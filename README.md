<div align="center">
  <img src="https://resolvingarchitecture.io/images/ra.png"  />

  <h1>Resolving Architecture</h1>

  <p>
    <strong>Clarity in Design</strong>
  </p>
  
  <h2>SEDA Bus</h2>
  
  <p>
   Staged Event-Driven Architecture Bus - A form of message bus avoiding the high overhead of thread-based concurrency models where channels get their own inbound and outbound queues. 
  </p>
  
  <p>
    <a href="https://travis-ci.com/resolvingarchitecture/seda-bus"><img alt="build" src="https://img.shields.io/travis/resolvingarchitecture/seda-bus"/></a>
    <a href="https://crates.io/crates/seda-bus"><img alt="Crate Info" src="https://img.shields.io/crates/v/seda-bus.svg"/></a>
    <a href="https://docs.rs/crate/seda-bus/"><img alt="API Docs" src="https://img.shields.io/badge/docs.seda-bus-green"/></a>
  </p>
  <p>
    <a href="https://github.com/resolvingarchitecture/seda-bus/blob/master/LICENSE"><img alt="License" src="https://img.shields.io/github/license/resolvingarchitecture/seda-bus"/></a>
    <a href="https://resolvingarchitecture.io/ks/publickey.brian@resolvingarchitecture.io.asc"><img alt="PGP" src="https://img.shields.io/keybase/pgp/objectorange"/></a>
  </p>
  <p>
    <img alt="commits" src="https://img.shields.io/crates/d/seda-bus"/>
    <img alt="repo size" src="https://img.shields.io/github/repo-size/resolvingarchitecture/seda-bus"/>
  </p>
  <p>
    <img alt="num lang" src="https://img.shields.io/github/languages/count/resolvingarchitecture/seda-bus"/>
    <img alt="top lang" src="https://img.shields.io/github/languages/top/resolvingarchitecture/seda-bus"/>
    <a href="https://blog.rust-lang.org/2020/03/12/Rust-1.42.html"><img alt="Rustc Version 1.42+" src="https://img.shields.io/badge/rustc-1.42+-green.svg"/></a>
  </p>

  <h4>
    <a href="https://resolvingarchitecture.io">Info</a>
    <span> | </span>
    <a href="https://docs.rs/crate/seda-bus/">Docs</a>
    <span> | </span>
    <a href="https://github.com/resolvingarchitecture/seda-bus/blob/master/CHANGELOG.md">Changelog</a>
  </h4>
</div>

## Donate
Request BTC/XMR address for a donation at brian@resolvingarchitecture.io.

## Notes
!! WIP - not stable until version 1.0 !!

## Roadmap 

*[ ] 1.0.0 - Minimal Stable Useful Functionality
    *[x] 0.1.0 - send/receive non-persistent messages between two channels
    *[x] 0.2.0 - provide CLI
    *[ ] 0.3.0 - provide optional guaranteed delivery at the message level
*[ ] 2.0.0 - use [dbus](https://en.wikipedia.org/wiki/D-Bus) for inter-process communications on Linux
*[ ] 3.0.0 - use [ipcd](https://dev.to/legolord208/programming-for-redox-os-4124) for inter-process communications on RedoxOS

[Crates.io](https://crates.io/crates/seda_bus)

!! WIP - not stable until version 1.0 !!

## Background
Staged Event-Driven Architecture (SEDA) is an approach to software architecture that decomposes a complex,
event-driven application into a set of stages connected by queues. It avoids the high overhead associated
with thread-based concurrency models (i.e. locking, unlocking, and polling for locks), and decouples event
and thread scheduling from application logic. By performing admission control on each event queue, the
service can be well-conditioned to load, preventing resources from being over-committed when demand exceeds
service capacity.

SEDA employs dynamic control to automatically tune runtime parameters (such as the scheduling parameters of
each stage) as well as to manage load (like performing adaptive load shedding). Decomposing services into a
set of stages also enables modularity and code reuse, as well as the development of debugging tools for
complex event-driven applications.

A Bus type architectural router style is decentralized in nature such that each instance of a node can be used
with other bus nodes, messages need to go through any specific node as in more centralized routers like
hub-and-spoke type routers. This is accomplished by supporting publishers and consumers using their own addressing
schemes creating their own mappings through chosen message channels, e.g. one message channel could be associated
with a persistence type service while another with a network type service - the bus cares not which is called
they're all just endpoints.

Bringing together SEDA and Bus architectural patterns is what this component attempts.

This component is also implemented in [Java](https://github.com/resolvingarchitecture/seda-bus-java) and [Typescript](https://github.com/resolvingarchitecture/seda-bus-ts).
This project is meant to support the design of the Java SEDA Bus in an implementation closer to the Linux/Redox Operating System
for embedded projects. It may eventually support integration with each operating system's own messaging bus.

## Setup - Ubuntu 18.04
1. Install Rust
   ```shell script
   sudo apt update
   sudo apt upgrade
   curl https://sh.rustup.rs -sSf | sh
   ```
2. Restart terminal
3. Verify Rust installed
    ```shell script
     rustc --version
    ```
4. Install build essentials
    ```shell script
    sudo apt install build-essential
    ```
5. install crate
    ```shell script
    cargo install seda_bus
    ```
6. 

## Development

### Links
* https://github.com/diwic/dbus-rs
* https://crates.io/crates/dbus
