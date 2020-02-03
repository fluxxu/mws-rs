//! Amazon MWS Products API - Version 2011-10-01
//!
//! [Reference](http://docs.developer.amazonservices.com/en_US/products/Products_Overview.html)

use client::{Client, Method};
use result::MwsResult;

pub mod types;
pub use self::types::*;

static PATH: &'static str = "/Products/2011-10-01";
static VERSION: &'static str = "2011-10-01";

#[derive(FromXmlStream, Default, Debug)]
#[allow(non_snake_case)]
pub struct GetLowestPricedOffersForSKUResponse {
  pub Identifier: Identifier,
  pub Summary: Summary,
  pub Offers: Vec<Offer>,
}

response_envelope_type!(
  GetLowestPricedOffersForSKUResponseEnvelope<GetLowestPricedOffersForSKUResponse>,
  "GetLowestPricedOffersForSKUResponse",
  "GetLowestPricedOffersForSKUResult"
);

#[allow(non_snake_case)]
#[derive(Debug, Default, Serialize, SerializeMwsParams)]
pub struct GetLowestPricedOffersForSKUParameters {
  pub MarketplaceId: String,
  pub SellerSKU: String,
  pub ItemCondition: ItemCondition,
}

#[allow(non_snake_case)]
pub fn GetLowestPricedOffersForSKU(
  client: &Client,
  params: GetLowestPricedOffersForSKUParameters,
) -> MwsResult<GetLowestPricedOffersForSKUResponse> {
  client
    .request_xml_with_form(
      Method::Post,
      PATH,
      VERSION,
      "GetLowestPricedOffersForSKU",
      params,
    )
    .map(|e: GetLowestPricedOffersForSKUResponseEnvelope| e.into_inner())
    .map_err(|err| err.into())
}
