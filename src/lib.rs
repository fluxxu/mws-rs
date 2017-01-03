extern crate url;
extern crate chrono;
extern crate crypto;
extern crate reqwest;
extern crate base64;
#[macro_use] extern crate error_chain;
extern crate xml as xml_rs;

#[macro_use] mod macros;
pub mod client;
mod xml;
mod sign;

pub mod products;

pub struct Client {

}
