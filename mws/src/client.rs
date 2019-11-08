use reqwest;
pub use reqwest::header::ContentType;
use reqwest::Response;
pub use reqwest::{Method, StatusCode};
use result::{MwsError, MwsResult};
use sign::SignatureV2;
use std::io::Read;
use xmlhelper::decode::{FromXmlStream, Stream};
use SerializeMwsParams;

#[derive(Debug)]
pub struct ErrorResponse {
  pub status: StatusCode,
  pub info: Option<ErrorResponseInfo>,
  pub raw: String,
}

#[derive(Debug, Default)]
pub struct ErrorResponseInfo {
  pub errors: Vec<ErrorResponseError>,
  pub request_id: String,
}

#[derive(Debug, Default, PartialEq)]
pub struct ErrorResponseError {
  pub error_type: String,
  pub code: String,
  pub message: String,
  pub detail: String,
}

impl ErrorResponseInfo {
  fn from_xml_stream<R: ::std::io::Read>(s: &mut Stream<R>) -> MwsResult<ErrorResponseInfo> {
    use xmlhelper::decode::{characters, element, fold_elements, start_document};
    start_document(s)?;
    element(s, "ErrorResponse", |s| {
      fold_elements(s, ErrorResponseInfo::default(), |s, resp| {
        match s.local_name() {
          "Error" => {
            let err = fold_elements(s, ErrorResponseError::default(), |s, err| {
              match s.local_name() {
                "Type" => {
                  err.error_type = characters(s)?;
                }
                "Code" => {
                  err.code = characters(s)?;
                }
                "Message" => {
                  err.message = characters(s)?;
                }
                "Detail" => {
                  err.detail = characters(s)?;
                }
                _ => {}
              }
              Ok(())
            })?;
            resp.errors.push(err);
          }
          "RequestID" => {
            resp.request_id = characters(s)?;
          }
          _ => {}
        }
        Ok(())
      })
    })
    .into()
  }
}

impl FromXmlStream<Stream<reqwest::Response>> for ErrorResponseInfo {
  fn from_xml(s: &mut Stream<reqwest::Response>) -> MwsResult<ErrorResponseInfo> {
    ErrorResponseInfo::from_xml_stream(s)
  }
}

impl FromXmlStream<Stream<::std::io::Cursor<String>>> for ErrorResponseInfo {
  fn from_xml(s: &mut Stream<::std::io::Cursor<String>>) -> MwsResult<ErrorResponseInfo> {
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
  http_client: reqwest::Client,
}

impl Client {
  pub fn new(options: ClientOptions) -> MwsResult<Client> {
    Ok(Client {
      options: options,
      http_client: reqwest::Client::new(),
    })
  }

  pub fn with_http_client(options: ClientOptions, http_client: reqwest::Client) -> Client {
    Client {
      options: options,
      http_client: http_client,
    }
  }

  pub fn request<P>(
    &self,
    method: Method,
    path: &str,
    version: &str,
    action: &str,
    parameters: P,
  ) -> MwsResult<reqwest::Response>
  where
    P: SerializeMwsParams,
  {
    let mut sign = SignatureV2::new(
      &self.options.endpoint,
      &self.options.aws_access_key_id,
      &self.options.secret_key,
      self.options.mws_auth_token.as_ref().map(AsRef::as_ref),
    );
    for (k, v) in parameters.into_mws_params() {
      sign.add(&k, v);
    }
    sign.add("SellerId", &self.options.seller_id);
    //sign.add("Merchant", self.options.seller_id.as_ref());
    let url = sign
      .generate_url(method.clone(), path, version, action)?
      .to_string();
    //println!("request: {}", url);
    self
      .http_client
      .request(method, &url)
      .send()
      .map_err(MwsError::from)
      .and_then(handle_error_status)
  }

  pub fn request_with_body<P, R>(
    &self,
    method: Method,
    path: &str,
    version: &str,
    action: &str,
    parameters: P,
    body: R,
    content_md5: String,
    content_type: ContentType,
  ) -> MwsResult<reqwest::Response>
  where
    P: SerializeMwsParams,
    R: Read + Send + 'static,
  {
    let mut sign = SignatureV2::new(
      &self.options.endpoint,
      &self.options.aws_access_key_id,
      &self.options.secret_key,
      self.options.mws_auth_token.as_ref().map(AsRef::as_ref),
    );
    for (k, v) in parameters.into_mws_params() {
      sign.add(&k, v);
    }
    sign.add("SellerId", &self.options.seller_id);
    sign.add("ContentMD5Value", content_md5);
    //sign.add("Merchant", self.options.seller_id.as_ref());
    let url = sign
      .generate_url(method.clone(), path, version, action)?
      .to_string();
    //println!("request: {}", url);

    self
      .http_client
      .request(method, &url)
      .header(content_type)
      .body(reqwest::Body::new(body))
      .send()
      .map_err(MwsError::from)
      .and_then(handle_error_status)
  }

  pub fn request_xml<P, T>(
    &self,
    method: Method,
    path: &str,
    version: &str,
    action: &str,
    parameters: P,
  ) -> MwsResult<T>
  where
    P: SerializeMwsParams,
    T: FromXmlStream<Stream<reqwest::Response>>,
  {
    let resp = self.request(method, path, version, action, parameters)?;
    let mut stream = Stream::new(resp);
    let v = T::from_xml(&mut stream)?;
    Ok(v)
  }

  pub fn request_xml_with_body<P, R, T>(
    &self,
    method: Method,
    path: &str,
    version: &str,
    action: &str,
    parameters: P,
    body: R,
    content_md5: String,
    content_type: ContentType,
  ) -> MwsResult<T>
  where
    P: SerializeMwsParams,
    T: FromXmlStream<Stream<reqwest::Response>>,
    R: Read + Send + 'static,
  {
    let resp = self.request_with_body(
      method,
      path,
      version,
      action,
      parameters,
      body,
      content_md5,
      content_type,
    )?;
    let mut stream = Stream::new(resp);
    let v = T::from_xml(&mut stream)?;
    Ok(v)
  }

  #[cfg(test)]
  pub fn request_raw<P>(
    &self,
    method: Method,
    path: &str,
    version: &str,
    action: &str,
    parameters: P,
  ) -> MwsResult<(StatusCode, String)>
  where
    P: SerializeMwsParams,
  {
    let mut sign = SignatureV2::new(
      &self.options.endpoint,
      &self.options.aws_access_key_id,
      &self.options.secret_key,
      self.options.mws_auth_token.as_ref().map(AsRef::as_ref),
    );
    for (k, v) in parameters.into_mws_params() {
      sign.add(&k, v);
    }
    let url = sign
      .generate_url(method.clone(), path, version, action)?
      .to_string();
    let mut resp = self
      .http_client
      .request(method, &url)
      .send()
      .map_err(MwsError::from)
      .and_then(handle_error_status)?;
    let mut s = String::new();
    resp.read_to_string(&mut s)?;
    Ok((resp.status().clone(), s))
  }
}

fn handle_error_status(resp: Response) -> MwsResult<Response> {
  if resp.status().is_success() {
    Ok(resp)
  } else {
    use std::io::Cursor;

    let mut resp = resp;
    let mut body = String::new();
    resp.read_to_string(&mut body)?;
    let mut s = Stream::new(Cursor::new(body.clone()));
    match ErrorResponseInfo::from_xml(&mut s) {
      Ok(info) => Err(MwsError::ErrorResponse(ErrorResponse {
        status: resp.status().clone(),
        raw: body,
        info: Some(info),
      })),
      Err(_) => Err(MwsError::ErrorResponse(ErrorResponse {
        status: resp.status().clone(),
        raw: body,
        info: None,
      })),
    }
  }
}

#[cfg(test)]
pub fn get_test_client() -> Client {
  use std::env;
  Client::new(ClientOptions {
    endpoint: env::var("Endpoint").expect("get Endpoint"),
    seller_id: env::var("SellerId").expect("get SellerId"),
    mws_auth_token: None,
    aws_access_key_id: env::var("AWSAccessKeyId").expect("get AWSAccessKeyId"),
    secret_key: env::var("SecretKey").expect("get SecretKey"),
  })
  .expect("create client")
}

#[cfg(test)]
mod tests {
  use super::*;
  use dotenv::dotenv;

  #[test]
  #[ignore]
  fn it_works() {
    dotenv().ok();
    let client = get_test_client();
    let (status, body) = client
      .request_raw(
        Method::Post,
        "/Orders/2013-09-01",
        "2013-09-01",
        "GetServiceStatus",
        (),
      )
      .expect("send request");
    assert!(status.is_success());
    assert!(body.starts_with("<?xml"));

    use std::io::Cursor;
    let (status, body) = client
      .request_raw(
        Method::Post,
        "/Fake/2013-09-01",
        "2013-09-01",
        "GetServiceStatus",
        (),
      )
      .expect("send request");
    assert!(!status.is_success());
    let source = Cursor::new(body);
    let mut s = Stream::new(source);
    let err_info = ErrorResponseInfo::from_xml(&mut s).expect("decode error response");
    assert_eq!(err_info.errors.len(), 1);
    assert_eq!(
      err_info.errors[0],
      ErrorResponseError {
        error_type: "Sender".to_string(),
        code: "InvalidAddress".to_string(),
        message: "Section Fake/2013-09-01 is invalid".to_string(),
        detail: "".to_string(),
      }
    );
  }
}
