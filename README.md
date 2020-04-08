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

   