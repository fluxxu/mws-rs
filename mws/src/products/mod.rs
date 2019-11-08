//! Amazon MWS Products API - Version 2011-10-01
//!
//! [Reference](http://docs.developer.amazonservices.com/en_US/products/Products_Overview.html)

use result::MwsResult;

pub mod types;
pub use self::types::*;

static PATH: &'static str = "/Orders/2013-09-01";
static VERSION: &'static str = "2011-10-01";

#[allow(non_snake_case)]
#[derive(Debug, Default, Serialize, FromXmlStream)]
pub struct ListOrdersResponse {
  pub Orders: Vec<Order>,
  pub LastUpdatedBefore: Option<DateTime<Utc>>,
  pub CreatedBefore: Option<DateTime<Utc>>,
  pub NextToken: Option<String>,
}

/// Returns a list of products and their attributes, based on a list of ASIN, GCID, SellerSKU, UPC, EAN, ISBN, and JAN values.
///
/// The GetMatchingProductForId operation returns a list of products and their attributes,
/// based on a list of product identifier values that you specify.
/// Possible product identifiers are ASIN, GCID, SellerSKU, UPC, EAN, ISBN, and JAN.
///
/// # Restrictions
/// - The operation only returns product information if an active offer exists when IdType input is GCID, UPC, EAN, ISBN, or JAN.
/// - The operation always returns product information if IdType input is ASIN.
/// - The operation always returns product information when IdType input is SellerSKU and the seller has not deleted the offer. The offer can be inactive or active, but must exist.
///
/// [Reference](http://docs.developer.amazonservices.com/en_US/products/Products_GetMatchingProductForId.html)
#[allow(non_snake_case)]
pub fn GetMatchingProductForId<T: AsRef<str>>(
  id_type: Id,
  id_list: &[T],
) -> MwsResult<Vec<Product>> {
  unimplemented!()
}
