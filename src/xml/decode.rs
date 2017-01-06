//! Xml Decode

use std::io::Read;
pub use xmlhelper::decode::{Error, Result, Stream};

pub trait FromXML {
  type R: Read;

  fn from_xml(stream: Stream<Self:R>) -> Result<Self>;
}

pub fn decode<T: FromXML<R = R>>(source: R) -> Result<T> {
  T::from_xml(Stream::new(source))
}

impl<R: Read> Into<Stream<R>> for R {
  fn into(self) -> Stream<R> {
    Stream::new(self)
  }
}
