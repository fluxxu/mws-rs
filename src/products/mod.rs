//! Amazon MWS Products API - Version 2011-10-01
//!
//! [Reference](http://docs.developer.amazonservices.com/en_US/products/Products_Overview.html)

pub mod types;

use self::types::product::{Id, Product};

error_chain! {
  
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
pub fn get_matching_product_for_id<T: AsRef<str>>(id_type: Id, id_list: &[T]) -> Result<Vec<Product>> {
  unimplemented!() 
}
