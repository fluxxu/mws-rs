pub use client::ErrorResponse as MwsErrorResponse;
use types::GenericXmlResponseParseError;

#[derive(Fail, Debug)]
pub enum MwsError {
  #[fail(display = "io error: {}", _0)]
  Io(#[cause] ::std::io::Error),
  #[fail(display = "http error: {}", _0)]
  Http(#[cause] ::reqwest::Error),
  #[fail(display = "xml reader error: {}", _0)]
  XmlReader(#[cause] ::xml::reader::Error),
  #[fail(display = "xml writer error: {}", _0)]
  XmlWriter(#[cause] ::xml::writer::Error),
  #[fail(display = "xml element parse error: {}", _0)]
  XmlElementParse(#[cause] ::xmltree::ParseError),
  #[fail(display = "generic xml response parse error: {}", _0)]
  GenericXmlResponseParse(#[cause] GenericXmlResponseParseError),
  #[fail(display = "csv error: {}", _0)]
  Csv(#[cause] ::csv::Error),
  #[fail(display = "utf8 error: {}", _0)]
  Utf8(#[cause] ::std::str::Utf8Error),
  #[fail(display = "MWS request is unsuccessful: {:?}", _0)]
  ErrorResponse(MwsErrorResponse),
  #[fail(display = "unexpected end of xml: {}", _0)]
  UnexpectedEndOfXml(String),
  #[fail(
    display = "unexpected xml event: expected '{}', found '{}'",
    expected, found
  )]
  UnexpectedXmlEvent { expected: String, found: String },
  #[fail(display = "parse string error: {} : {}", what, message)]
  ParseString { what: String, message: String },
  #[fail(display = "invalid path name: '{}'", _0)]
  InvalidPath(String),
  #[fail(display = "Content-MD5 header missing")]
  ContentMD5HeaderMissing,
  #[fail(display = "{}", _0)]
  Msg(String),
}

impl MwsError {
  /// The common response to a 500 or 503 service error is
  /// to try the request again. Such service errors are
  /// usually only temporary and will resolve themselves.
  pub fn should_try_again(&self) -> bool {
    match *self {
      MwsError::ErrorResponse(ref res) => {
        let code = res.status.as_u16();
        code >= 500 && code < 600
      }
      MwsError::Io(_) => true,
      _ => false,
    }
  }
}

macro_rules! impl_from {
  ($v:ident($t:ty)) => {
    impl From<$t> for MwsError {
      fn from(e: $t) -> Self {
        MwsError::$v(e)
      }
    }
  };
}

impl_from!(Io(::std::io::Error));
impl_from!(Http(::reqwest::Error));
impl_from!(XmlReader(::xml::reader::Error));
impl_from!(XmlWriter(::xml::writer::Error));
impl_from!(Csv(::csv::Error));
impl_from!(Utf8(::std::str::Utf8Error));
impl_from!(Msg(String));
impl_from!(XmlElementParse(::xmltree::ParseError));
impl_from!(GenericXmlResponseParse(GenericXmlResponseParseError));

pub type MwsResult<T> = Result<T, MwsError>;
