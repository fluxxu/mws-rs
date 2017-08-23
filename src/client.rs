use sign::{SignatureV2};
pub use reqwest::{Method, StatusCode};
pub use reqwest::header::ContentType;
use reqwest;
use xmlhelper::decode::{FromXMLStream, Stream};
use tdff::FromTdff;
use std::sync::{Arc, Mutex};
use std::io::Read;

error_chain! {
  foreign_links {
    Io(::std::io::Error);
    Request(reqwest::Error);
  }

  links {
    XmlDecode(::xmlhelper::decode::Error, ::xmlhelper::decode::ErrorKind);
    TdffDecode(::tdff::Error, ::tdff::ErrorKind);
    Sign(super::sign::Error, super::sign::ErrorKind);
  }

  errors {
    ErrorResponse(resp: ErrorResponse) {
      description("MWS request is unsuccessful")
      display("MWS request is unsuccessful: {:?}", resp)
    }
    PoisonError
  }
}

#[derive(Debug)]
pub enum Response<T> {
  Success(T),
  Error(ErrorResponse),
}

#[derive(Debug)]
pub struct ErrorResponse {
  pub status: StatusCode,
  pub info: Option<ErrorResponseInfo>,
  pub raw: String
}

#[derive(Debug, Default)]
pub struct ErrorResponseInfo {
  pub errors: Vec<ErrorResponseError>,
  pub request_id: String
}

#[derive(Debug, Default, PartialEq)]
pub struct ErrorResponseError {
  pub error_type: String,
  pub code: String,
  pub message: String,
  pub detail: String
}

impl ErrorResponseInfo {
  fn from_xml_stream<R: ::std::io::Read>(s: &mut Stream<R>) -> ::xmlhelper::decode::Result<ErrorResponseInfo> {
    use xmlhelper::decode::{start_document, element, fold_elements, characters};
    start_document(s)?;
    element(s, "ErrorResponse", |s| {
      fold_elements(s, ErrorResponseInfo::default(), |s, resp| {
        match s.local_name() {
          "Error" => {
            let err = fold_elements(s, ErrorResponseError::default(), |s, err| {
              match s.local_name() {
                "Type" => {
                  err.error_type = characters(s)?;
                },
                "Code" => {
                  err.code = characters(s)?;
                },
                "Message" => {
                  err.message = characters(s)?;
                },
                "Detail" => {
                  err.detail = characters(s)?;
                },
                _ => {}
              }
              Ok(())
            })?;
            resp.errors.push(err);
          },
          "RequestID" => {
            resp.request_id = characters(s)?;
          },
          _ => {}
        }
        Ok(())
      })
    }).into()
  }
}

impl FromXMLStream<Stream<reqwest::Response>> for ErrorResponseInfo {
  fn from_xml(s: &mut Stream<reqwest::Response>) -> ::xmlhelper::decode::Result<ErrorResponseInfo> {
    ErrorResponseInfo::from_xml_stream(s)
  }
}

impl FromXMLStream<Stream<::std::io::Cursor<String>>> for ErrorResponseInfo {
  fn from_xml(s: &mut Stream<::std::io::Cursor<String>>) -> ::xmlhelper::decode::Result<ErrorResponseInfo> {
    ErrorResponseInfo::from_xml_stream(s)
  }
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
  options: ClientOptions,
  http_client: Arc<Mutex<reqwest::Client>>,
}

macro_rules! get_http_client {
  ($s:ident) => ($s.http_client.lock().map_err(|_| ErrorKind::PoisonError)?)
}

impl Client {
  pub fn new(options: ClientOptions) -> Result<Client> {
    Ok(Client {
      options: options,
      http_client: Arc::new(Mutex::new(reqwest::Client::new()?)),
    })
  }

  pub fn new_with_http_client(options: ClientOptions, http_client: Arc<Mutex<reqwest::Client>>) -> Client {
    Client {
      options: options,
      http_client: http_client,
    }
  }

  pub fn request<P>(&self, method: Method, path: &str, version: &str, action: &str, parameters: P) -> Result<reqwest::Response> 
    where P: Into<Vec<(String, String)>>
  {
    let mut sign = SignatureV2::new(self.options.endpoint.clone(), self.options.aws_access_key_id.clone(), self.options.secret_key.clone());
    for (k, v) in parameters.into() {
      sign.add(&k, v);
    }
    sign.add("SellerId", self.options.seller_id.as_ref());
    //sign.add("Merchant", self.options.seller_id.as_ref());
    let url = sign.generate_url(method.clone(), path, version, action)?.to_string();
    //println!("request: {}", url);
    get_http_client!(self).request(method, &url)?.send()
      .map_err(Into::into)
  }

  pub fn request_with_body<P, R>(&self, method: Method, path: &str, version: &str, action: &str, parameters: P, body: R, content_md5: String, content_type: ContentType) -> Result<reqwest::Response> 
    where P: Into<Vec<(String, String)>>,
          R: Read + Send + 'static
  {
    let mut sign = SignatureV2::new(self.options.endpoint.clone(), self.options.aws_access_key_id.clone(), self.options.secret_key.clone());
    for (k, v) in parameters.into() {
      sign.add(&k, v);
    }
    sign.add("SellerId", self.options.seller_id.as_ref());
    sign.add("ContentMD5Value", content_md5);
    //sign.add("Merchant", self.options.seller_id.as_ref());
    let url = sign.generate_url(method.clone(), path, version, action)?.to_string();
    //println!("request: {}", url);

    get_http_client!(self).request(method, &url)?
      .header(content_type)
      .body(reqwest::Body::new(body))
      .send()
      .map_err(Into::into)
  }

  pub fn request_xml<P, T>(&self, method: Method, path: &str, version: &str, action: &str, parameters: P) -> Result<Response<T>>
    where P: Into<Vec<(String, String)>>, T: FromXMLStream<Stream<reqwest::Response>>
  {
    let mut resp = self.request(method, path, version, action, parameters)?;
    if resp.status().is_success() {
      let mut stream = Stream::new(resp);
      let v = T::from_xml(&mut stream)?;
      Ok(Response::Success(v))
    } else {
      use std::io::{Read, Cursor};

      let mut body = String::new();
      resp.read_to_string(&mut body)?;
      let mut s = Stream::new(Cursor::new(body.clone()));
      match ErrorResponseInfo::from_xml(&mut s) {
        Ok(info) => Ok(Response::Error(ErrorResponse {
          status: resp.status().clone(),
          raw: body,
          info: Some(info),
        })),
        Err(_) => Ok(Response::Error(ErrorResponse {
          status: resp.status().clone(),
          raw: body,
          info: None,
        })),
      }
    }
  }

  pub fn request_xml_with_body<P, R, T>(&self, method: Method, path: &str, version: &str, action: &str, parameters: P, body: R, content_md5: String, content_type: ContentType) -> Result<Response<T>>
    where P: Into<Vec<(String, String)>>, T: FromXMLStream<Stream<reqwest::Response>>,
          R: Read + Send + 'static
  {
    let mut resp = self.request_with_body(method, path, version, action, parameters, body, content_md5, content_type)?;
    if resp.status().is_success() {
      let mut stream = Stream::new(resp);
      let v = T::from_xml(&mut stream)?;
      Ok(Response::Success(v))
    } else {
      use std::io::{Read, Cursor};

      let mut body = String::new();
      resp.read_to_string(&mut body)?;
      let mut s = Stream::new(Cursor::new(body.clone()));
      match ErrorResponseInfo::from_xml(&mut s) {
        Ok(info) => Ok(Response::Error(ErrorResponse {
          status: resp.status().clone(),
          raw: body,
          info: Some(info),
        })),
        Err(_) => Ok(Response::Error(ErrorResponse {
          status: resp.status().clone(),
          raw: body,
          info: None,
        })),
      }
    }
  }

  pub fn request_tdff<P, T>(&self, method: Method, path: &str, version: &str, action: &str, parameters: P) -> Result<Response<T>>
    where P: Into<Vec<(String, String)>>, T: FromTdff<reqwest::Response>
  {
    let mut resp = self.request(method, path, version, action, parameters)?;
    if resp.status().is_success() {
      let v = T::from_tdff(resp)?;
      Ok(Response::Success(v))
    } else {
      use std::io::{Read, Cursor};

      let mut body = String::new();
      resp.read_to_string(&mut body)?;
      let mut s = Stream::new(Cursor::new(body.clone()));
      match ErrorResponseInfo::from_xml(&mut s) {
        Ok(info) => Ok(Response::Error(ErrorResponse {
          status: resp.status().clone(),
          raw: body,
          info: Some(info),
        })),
        Err(_) => Ok(Response::Error(ErrorResponse {
          status: resp.status().clone(),
          raw: body,
          info: None,
        })),
      }
    }
  }

  #[cfg(test)]
  pub fn request_raw<P>(&self, method: Method, path: &str, version: &str, action: &str, parameters: P) -> Result<(StatusCode, String)>
    where P: Into<Vec<(String, String)>>
  {
    use std::io::Read;

    let mut sign = SignatureV2::new(self.options.endpoint.clone(), self.options.aws_access_key_id.clone(), self.options.secret_key.clone());
    for (k, v) in parameters.into() {
      sign.add(&k, v);
    }
    let url = sign.generate_url(method.clone(), path, version, action)?.to_string();
    let mut resp = get_http_client!(self).request(method, &url).send()?;
    let mut s = String::new();
    resp.read_to_string(&mut s)?;
    Ok((resp.status().clone(), s))
  }
}

#[cfg(test)]
pub fn get_test_client() -> Client {
  use std::env;
  Client::new(ClientOptions {
    endpoint: env::var("Endpoint").expect("get Endpoint"),
    seller_id: env::var("SellerId").expect("get SellerId"),
    mws_auth_token: None,
    aws_access_key_id : env::var("AWSAccessKeyId").expect("get AWSAccessKeyId"),
    secret_key: env::var("SecretKey").expect("get SecretKey"),
  }).expect("create client")
}

#[cfg(test)]
mod tests {
  use dotenv::dotenv;
  use super::*;

  #[test]
  fn it_works() {
    dotenv().ok();
    let client = get_test_client();
    let (status, body) = client.request_raw(Method::Post, "/Orders/2013-09-01", "2013-09-01", "GetServiceStatus", vec![]).expect("send request");
    assert!(status.is_success());
    assert!(body.starts_with("<?xml"));

    use std::io::Cursor;
    let (status, body) = client.request_raw(Method::Post, "/Fake/2013-09-01", "2013-09-01", "GetServiceStatus", vec![]).expect("send request");
    assert!(!status.is_success());
    let source = Cursor::new(body);
    let mut s = Stream::new(source);
    let err_info = ErrorResponseInfo::from_xml(&mut s).expect("decode error response");
    assert_eq!(err_info.errors.len(), 1);
    assert_eq!(err_info.errors[0], ErrorResponseError {
      error_type: "Sender".to_string(), code: "InvalidAddress".to_string(), message: "Section Fake/2013-09-01 is invalid".to_string(), detail: "".to_string()
    });
  }
}