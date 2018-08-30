//! Tab-delimited flat file helpers

use csv::Reader;
use result::{MwsError, MwsResult};
use std::io::Read;

pub trait FromTdff<R: Read>: Sized {
  fn from_tdff(source: R) -> MwsResult<Self>;
}

pub struct TdffScanner<R: Read> {
  headers: Vec<String>,
  reader: Reader<R>,
}

impl<R: Read> TdffScanner<R> {
  pub fn new(source: R) -> MwsResult<TdffScanner<R>> {
    let mut reader = Reader::from_reader(source).delimiter(b'\t');
    Ok(TdffScanner {
      headers: reader.headers()?,
      reader: reader,
    })
  }
}

pub type TdffRow<'a> = Vec<(&'a str, String)>;

impl<R: Read> TdffScanner<R> {
  pub fn for_each_row<'a, F>(&'a mut self, mut f: F) -> MwsResult<()>
  where
    F: FnMut(usize, TdffRow<'a>) -> MwsResult<()> + 'a,
  {
    let size = self.headers.len();
    for (row_i, row) in self.reader.records().enumerate() {
      let mut row_container = Vec::with_capacity(size);
      for (i, value) in row?.into_iter().enumerate() {
        match self.headers.get(i) {
          Some(key) => {
            row_container.push((key.as_ref() as &str, value));
          }
          None => {}
        }
      }
      f(row_i, row_container)?;
    }
    Ok(())
  }
}

pub trait FromTdffField: Sized {
  fn parse_tdff_field(key: &str, v: &str) -> MwsResult<Self>;
}

impl<T, Err> FromTdffField for T
where
  T: ::std::str::FromStr<Err = Err>,
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
      Err(MwsError::ParseString {
        what: key.to_string(),
        message: "value is empty".to_string(),
      })
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use std::io::Cursor;

  #[test]
  fn test_tdff() {
    let source = Cursor::new(r#"settlement-id	settlement-start-date	settlement-end-date	deposit-date	total-amount	currency	transaction-type	order-id	merchant-order-id	adjustment-id	shipment-id	marketplace-name	shipment-fee-type	shipment-fee-amount	order-fee-type	order-fee-amount	fulfillment-id	posted-date	order-item-code	merchant-order-item-id	merchant-adjustment-item-id	sku	quantity-purchased	price-type	price-amount	item-related-fee-type	item-related-fee-amount	misc-fee-amount	other-fee-amount	other-fee-reason-description	promotion-id	promotion-type	promotion-amount	direct-payment-type	direct-payment-amount	other-amount
6016502941	2015-12-02T03:27:08+00:00	2015-12-16T03:27:08+00:00	2015-12-18T03:27:08+00:00	16630.13	USD																														
6016502941						Order	112-8095165-5463447	112-8095165-5463447		D1FyNyr6b	Amazon.com					AFN	2015-12-02T04:15:26+00:00	62538071274626			edifier-p270-gold	1													
6016502941						Order	112-8095165-5463447	112-8095165-5463447		D1FyNyr6b	Amazon.com					AFN	2015-12-02T04:15:26+00:00	62538071274626			edifier-p270-gold		Principal	27.99											
6016502941						Order	112-8095165-5463447	112-8095165-5463447		D1FyNyr6b	Amazon.com					AFN	2015-12-02T04:15:26+00:00	62538071274626			edifier-p270-gold				FBAPerOrderFulfillmentFee	-1.00									
"#);
    let mut scanner = TdffScanner::new(source).expect("new scanner");
    let mut rows = vec![];
    scanner
      .for_each_row(|_, row| {
        rows.push(
          row
            .into_iter()
            .map(|(k, v)| (k.to_string(), v.clone()))
            .collect::<Vec<_>>(),
        );
        Ok(())
      }).expect("for_each_row");
    assert_eq!(rows.len(), 4);
    assert_eq!(
      rows[0][0],
      ("settlement-id".to_string(), "6016502941".to_string())
    );
    assert_eq!(
      rows[1][6],
      ("transaction-type".to_string(), "Order".to_string())
    );
    assert_eq!(
      rows[3].iter().find(|t| t.0 == "sku").expect("sku column").1,
      "edifier-p270-gold".to_string()
    );
  }
}
