use sign::{SignatureV2};
pub use reqwest::{Method, Response};
use xmlhelper::decode::{FromXMLStream, XmlEventStream};

error_chain! {
}

pub struct ErrorResponse {
  pub error_type: String,
  pub error_code: String,
  pub error_message: String,
  pub request_id: String,
}

/// [Reference](http://docs.developer.amazonservices.com/en_CA/dev_guide/DG_Endpoints.html)
pub struct ClientOptions {
  /// Your software can access Amazon Marketplace Web Service (Amazon MWS) using region-specific endpoints.
  pub endpoint: String,

  /// Your seller or merchant identifier.
  pub seller_id: String,

  /// Represents the authorization of a specific developer of a web application by a specific Amazon seller.
  pub mws_auth_token: Option<String>,

  /// Your Amazon MWS account is identified by your access key Id, which Amazon MWS uses to look up your Secret Access Key.
  pub aws_access_key_id: String,
  pub secret_key: String,
}

pub struct Client {
  sign: SignatureV2,
}

impl Client {
  pub fn new(options: ClientOptions) -> Client {
    Client {
      sign: SignatureV2::new(options.endpoint, options.aws_access_key_id, options.secret_key),
    }
  }

  pub fn request<P, S, T>(&self, method: Method, path: &str, parameters: P) -> Result<T>
    where P: Into<Vec<(String, String)>>, S: XmlEventStream, T: FromXMLStream<S>
  {
    Ok(T::default())
  } 
}