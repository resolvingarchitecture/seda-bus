extern crate log;
extern crate simple_logger;

use log::{trace,info};

use ra_common::models::{Envelope,LifeCycle,Route};
use seda_bus::bus::MessageBus;
use seda_bus::endpoint::MessageEndpoint;

#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}
