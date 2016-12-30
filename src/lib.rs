extern crate url;
extern crate chrono;
extern crate crypto;
extern crate reqwest;
extern crate base64;
#[macro_use] extern crate error_chain;
extern crate xml;

#[macro_use] mod macros;
pub mod client;
mod serialize;
mod sign;

pub mod products;

pub struct Client {

}
