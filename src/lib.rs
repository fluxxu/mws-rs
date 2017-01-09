#![feature(log_syntax)]

extern crate url;
extern crate chrono;
extern crate crypto;
extern crate reqwest;
extern crate base64;
#[macro_use] extern crate error_chain;
extern crate xmlhelper;

#[cfg(test)] extern crate dotenv;

#[macro_use] mod macros;
pub mod client;
mod sign;

// pub mod products;
pub mod orders;