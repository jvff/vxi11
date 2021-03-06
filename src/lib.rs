#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate futures;
#[macro_use]
extern crate onc_rpc;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_xdr;
extern crate tokio_core;
extern crate tokio_proto;
extern crate tokio_service;

mod device;

pub mod rpc;

pub use self::device::Device;
