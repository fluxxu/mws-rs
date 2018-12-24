//! Client library for Amazon Marketplace Web Service (Amazon MWS)
//!

// #![deny(warnings)]

extern crate base64;
extern crate chrono;
extern crate crypto;
extern crate failure;
extern crate reqwest;
extern crate url;
extern crate xml;
#[macro_use]
extern crate failure_derive;
extern crate csv;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate mws_derive;

pub use mws_derive::FromTdffRow;

#[cfg(test)]
extern crate dotenv;
#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;
#[macro_use]
extern crate lazy_static;

pub mod result;
#[macro_use]
pub mod xmlhelper;
mod types;
#[macro_use]
mod macros;
#[macro_use]
pub mod tdff;
pub mod client;
pub mod constants;
mod sign;

// pub mod products;
pub mod feeds;
pub mod fulfillment_inbound_shipment;
pub mod fulfillment_inventory;
pub mod fulfillment_outbound;
pub mod merchant_fulfillment;
pub mod orders;
pub mod reports;

pub use self::types::{ResponseEnvelope, SerializeMwsParams, SerializeMwsParamsContext};
