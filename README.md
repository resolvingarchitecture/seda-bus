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
Request BTC/XMR/ZEC address for a donation at brian@resolvingarchitecture.io.

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
   