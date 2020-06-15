extern crate log;
extern crate simple_logger;

use log::{trace,info};
use std::{thread, env};
use std::time::Duration;
use seda_bus::{MessageBus, BusType};
use ra_common::models::Envelope;
use clap::{crate_version, App, AppSettings, Arg};

fn main() {
    simple_logger::init().unwrap();
    let m = App::new("seda_bus")
        .about("Staged Event-Driven Architecture Bus - A form of message bus avoiding the high overhead of thread-based concurrency models where channels get their own inbound and outbound queues.")
        .version(crate_version!())
        .author("Brian Taylor <brian@resolvingarchitecture.io>")
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::ColorAlways)
        .arg(
            Arg::with_name("name")
                .help("The name of the bus to instantiate.")
                .short("n")
                .long("name")
                .required(true)
                .takes_value(true)
        )
        .get_matches();
    let bus_name = String::from(m.value_of("name").unwrap());

    trace!("Starting RA SEDA Bus...");

    let mut bus = MessageBus::new(String::from(bus_name)).unwrap();
    let from = bus.register();
    let to = bus.register();

    for n in 1..10 {
        bus.send(Envelope::new(from,to, format!("Hello 12: {}",n).into_bytes()));
    }

    thread::spawn( move || {
        loop {
            match bus.poll_wait(to, 100) {
                Some(env) => info!("env from={} to={} msg={}", env.from, env.to, String::from_utf8(env.msg).unwrap()),
                None => info!("x")
            }
        }
    });

    thread::sleep(Duration::from_secs(1));

    trace!("SED Bus Stopped.");
}
