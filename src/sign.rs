//! Implements AWS Signature Version 2 Signing
//!
//! http://docs.aws.amazon.com/general/latest/gr/signature-version-2.html
//!

use std::cmp::{Ord, Ordering};
pub use reqwest::Method;

/// Query string parameter key/value pair
#[derive(Debug, Eq, PartialEq, PartialOrd, Clone)]
pub struct Param (String, String);

impl Ord for Param {
  fn cmp(&self, other: &Self) -> Ordering {
    self.0.cmp(&other.0)
  }
}

/// Signature V2 generator
#[derive(Debug, Clone)]
pub struct SignatureV2 {
  host: String,
  secret_key: String,
  pairs: Vec<Param>,
}

impl SignatureV2 {
  /// Constructs a new, empty generator
  pub fn new<T: Into<String>>(host:T, secret_key: T) -> SignatureV2 {
    SignatureV2 {
      host: host.into(),
      secret_key: secret_key.into(),
      pairs: Vec::new(),
    }
  }

  /// Adds a key/value pair. Duplicated key is overridden.
  pub fn add<T: Into<String>>(&mut self, key: &str, value: T) -> &mut Self {
    SignatureV2::set_param(&mut self.pairs, key, value);
    self
  }

  fn set_param<T: Into<String>(params: &mut Vec<Param>, key: &str, value: T) {
    match params.iter().position(|&Param (ref k, _)| k == key) {
      Some(pos) => {
        params[pos].1 = value.into();
      },
      None => {
        params.push(Param (key.to_owned(), value.into()));
      }
    }
  }

  /// Generates a https url which contains all parameters and signed by HMAC-SHA256
  pub fn generate_url<'a, 'b: 'a, T: Into<&'b str>>(&'a self, method: Method, action: T) -> String {
    use url::percent_encoding::{percent_encode, QUERY_ENCODE_SET};

    let mut params = self.pairs.clone();
    let mut qs = String::with_capacity(255);

    SignatureV2::set_param(&mut params, "Version", "2011-10-01");
    SignatureV2::set_param(&mut params, "Action", action);

    params.sort();
    for Param(ref key, ref value) in params {
      if qs.len() > 0 {
        qs.push_str("&");
      }

      for part in percent_encode(key.as_bytes(), QUERY_ENCODE_SET) {
        qs.push_str(part);
      }

      qs.push_str("=");

      for part in percent_encode(value.as_bytes(), QUERY_ENCODE_SET) {
        qs.push_str(part);
      }
    }
    qs
  }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}