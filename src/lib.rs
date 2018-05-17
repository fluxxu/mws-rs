//! Client library for Amazon Marketplace Web Service (Amazon MWS)
//!

extern crate base64;
extern crate chrono;
extern crate crypto;
extern crate reqwest;
extern crate url;
extern crate xml;
#[macro_use]
extern crate error_chain;
extern crate csv;
extern crate serde;
#[macro_use]
extern crate serde_derive;

#[cfg(test)]
extern crate dotenv;

#[macro_use]
pub mod xmlhelper;
mod types;
#[macro_use]
mod macros;
#[macro_use]
pub mod tdff;
pub mod client;
mod sign;

// pub mod products;
pub mod feeds;
pub mod fulfillment_inbound_shipment;
pub mod fulfillment_inventory;
pub mod fulfillment_outbound;
pub mod orders;
pub mod reports;
