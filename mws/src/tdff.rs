//! Tab-delimited flat file helpers

use csv::Reader;
use result::{MwsError, MwsResult};
use std::io::Read;

pub struct TdffParser<R: Read> {
  headers: Vec<String>,
  reader: Reader<R>,
}

impl<R: Read> TdffParser<R> {
  pub fn new(source: R) -> MwsResult<TdffParser<R>> {
    let mut reader = Reader::from_reader(source).delimiter(b'\t');
    Ok(TdffParser {
      headers: reader.headers()?,
      reader: reader,
    })
  }
}

pub type TdffRow<'a> = Vec<(&'a str, String)>;

pub trait FromTdffRow: Default + Sized {
  fn from_tdff_row(r: &TdffRow) -> MwsResult<Self>;
}

impl<R: Read> TdffParser<R> {
  pub fn parse<'a, T>(&'a mut self) -> MwsResult<Vec<T>>
  where
    T: FromTdffRow,
  {
    let size = self.headers.len();
    let mut row_container = Vec::with_capacity(size);
    let mut items = vec![];
    for row in self.reader.records() {
      row_container.clear();
      for (i, value) in row?.into_iter().enumerate() {
        match self.headers.get(i) {
          Some(key) => {
            row_container.push((key.as_ref() as &str, value));
          }
          None => {}
        }
      }
      items.push(T::from_tdff_row(&row_container)?);
    }
    Ok(items)
  }
}

pub trait FromTdffField: Sized {
  fn parse_tdff_field(key: &str, v: &str) -> MwsResult<Self>;
}

impl<T, Err> FromTdffField for T
where
  T: ::std::str::FromStr<Err = Err> + Default,
  Err: ::std::error::Error,
{
  fn parse_tdff_field(key: &str, v: &str) -> MwsResult<Self> {
    let trimmed = v.trim();
    if !trimmed.is_empty() {
      trimmed.parse().map_err(|err| MwsError::ParseString {
        what: key.to_string(),
        message: format!("{}: '{}'", err, v),
      })
    } else {
      Ok(T::default())
    }
  }
}
