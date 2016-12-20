//! Implements AWS Signature Version 2 Signing
//!
//! http://docs.developer.amazonservices.com/en_CA/dev_guide/DG_SigningQueryRequest.html
//! http://docs.aws.amazon.com/general/latest/gr/signature-version-2.html
//!

use std::cmp::{Ord, Ordering};
pub use reqwest::Method;
use std::path::Path;
use url::percent_encoding::{percent_encode, EncodeSet};

error_chain! {
  errors {
    InvalidPath(path: String) {
      description("invalid path")
      display("invalid path name: '{}'", path)
    }
  }
}

/// URL encode the parameter name and values according to the following rules:
/// - Do not URL encode any of the unreserved characters that RFC 3986 defines. These unreserved characters are A-Z, a-z, 0-9, hyphen ( - ), underscore ( _ ), period ( . ), and tilde ( ~ ).
/// - Percent encode all other characters with %XY, where X and Y are hex characters 0-9 and uppercase A-F.
/// - Percent encode extended UTF-8 characters in the form %XY%ZA....
/// - Percent encode the space character as %20. Do not percent encode the space character as +, as some common encoding schemes do.
#[derive(Clone)]
struct ParameterEncodeSet;
impl EncodeSet for ParameterEncodeSet {
  fn contains(&self, byte: u8) -> bool {
    // 0-9
    if byte >= 0x30 && byte <= 0x39 {
      return false;
    }

    // A-Z
    if byte >= 0x41 && byte <= 0x5A {
      return false;
    }

    // a-z
    if byte >= 0x61 && byte <= 0x7A {
      return false;
    }

    // hyphen ( - ), underscore ( _ ), period ( . ), and tilde ( ~ )
    if byte == 0x2D || byte == 0x5F || byte == 0x2E || byte == 0x7E {
      return false;
    }

    true
  }
}

/// Query string parameter key/value pair
#[derive(Debug, Eq, PartialEq, PartialOrd, Clone)]
pub struct Param (String, String);

impl Ord for Param {
  fn cmp(&self, other: &Self) -> Ordering {
    self.0.cmp(&other.0)
  }
}

pub struct SignedUrl<'a> {
  pub host: &'a str,
  pub method: Method,
  pub path: String,
  pub query_string: String,
  pub signature: String,
}

impl<'a> Into<String> for SignedUrl<'a> {
  fn into(self) -> String {
    self.to_string()
  }
}

impl<'a> SignedUrl<'a> {
  pub fn to_string(self) -> String {
    let mut signature_encoded = String::with_capacity(((self.signature.len() as f32) * 1.4) as usize);
    for part in percent_encode(self.signature.as_bytes(), ParameterEncodeSet) {
      signature_encoded.push_str(part);
    }
    format!("https://{host}{path}?{qs}&Signature={signature}", host = &self.host, path = self.path, qs = self.query_string, signature = signature_encoded)
  }
}

/// Signature V2 generator
#[derive(Debug, Clone)]
pub struct SignatureV2 {
  host: String,
  aws_access_key_id: String,
  secret_key: String,
  pairs: Vec<Param>,
}

impl SignatureV2 {
  /// Constructs a new, empty generator
  pub fn new<T: Into<String>>(host:T, aws_access_key_id: T, secret_key: T) -> SignatureV2 {
    SignatureV2 {
      host: host.into(),
      aws_access_key_id: aws_access_key_id.into(),
      secret_key: secret_key.into(),
      pairs: Vec::new(),
    }
  }

  /// Adds a key/value pair. Duplicated key is overridden.
  pub fn add<T: Into<String>>(&mut self, key: &str, value: T) -> &mut Self {
    SignatureV2::set_param(&mut self.pairs, key, value);
    self
  }

  fn set_param<T: Into<String>>(params: &mut Vec<Param>, key: &str, value: T) {
    match params.iter().position(|&Param (ref k, _)| k == key) {
      Some(pos) => {
        params[pos].1 = value.into();
      },
      None => {
        params.push(Param (key.to_owned(), value.into()));
      }
    }
  }

  /// Generates a SignedUrl which contains all parameters and signed by HMAC-SHA256
  pub fn generate_url<'a, T: AsRef<str>, P: AsRef<Path>>(&'a self, method: Method, path: P, version: T, action: T) -> Result<SignedUrl<'a>> {
    use chrono::UTC;
    use crypto::hmac::Hmac;
    use crypto::sha2::Sha256;
    use crypto::mac::Mac;
    use base64;

    let mut params = self.pairs.clone();
    let mut qs = String::with_capacity(255);

    SignatureV2::set_param(&mut params, "AWSAccessKeyId", self.aws_access_key_id.as_ref());
    SignatureV2::set_param(&mut params, "SignatureMethod", "HmacSHA256");
    SignatureV2::set_param(&mut params, "SignatureVersion", "2");
    SignatureV2::set_param(&mut params, "Version", version.as_ref());
    SignatureV2::set_param(&mut params, "Action", action.as_ref());

    #[cfg(not(test))]
    SignatureV2::set_param(&mut params, "Timestamp", UTC::now().format("%+").to_string());

    params.sort();
    for Param(ref key, ref value) in params {
      if qs.len() > 0 {
        qs.push_str("&");
      }

      for part in percent_encode(key.as_bytes(), ParameterEncodeSet) {
        qs.push_str(part);
      }

      qs.push_str("=");

      for part in percent_encode(value.as_bytes(), ParameterEncodeSet) {
        qs.push_str(part);
      }
    }

    let path_str = path.as_ref().to_str().ok_or_else(|| {
      let display = path.as_ref().display();
      ErrorKind::InvalidPath(format!("{}", display))
    })?;
    let signature = {
      let canonical_qs = format!("{method}\n{host}\n{path}/{version}\n{qs}", method = &method, host = &self.host, path = path_str, version = version.as_ref(), qs = qs);
      println!("string to sign: {}", canonical_qs);
      let mut hmac = Hmac::new(Sha256::new(), self.secret_key.as_bytes());
      hmac.input(canonical_qs.as_bytes());
      base64::encode(&hmac.result().code())
    };

    Ok(SignedUrl {
      host: &self.host,
      method: method,
      path: path_str.to_string(),
      query_string: qs,
      signature: signature,
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let mut s = SignatureV2::new("mws.amazonservices.ca", "3333", "0000");
    let url = s
      .add("Timestamp", "2016-12-20T18:42:04Z")
      .add("MarketplaceId", "5555")
      .add("ASINList.ASIN.1", "6666")
      .add("SellerId", "1111")
      .generate_url(Method::Post, "/Products", "2011-10-01", "GetMatchingProduct").expect("generate url");

    assert_eq!(url.signature, "e7NJFMRLOOpRUp0IP42irtpKzq404KDbjZpKZ/OWRLI=");
  }
}