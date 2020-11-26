//! Amazon MWS Fulfillment Inventory API - Version 2010-10-01
//!
//! [Documentation](http://docs.developer.amazonservices.com/en_CA/fba_inventory/FBAInventory_Overview.html)

use chrono::{DateTime, Utc};
use client::{Client, Method};
mod types;
pub use self::types::{
  Condition, InventorySupply, InventorySupplyDetail, SupplyType, Timepoint, TimepointType,
};
use result::MwsResult;

static PATH: &'static str = "/FulfillmentInventory/2010-10-01";
static VERSION: &'static str = "2010-10-01";

#[derive(Debug, Serialize, SerializeMwsParams)]
pub enum ResponseGroup {
  Basic,
  Detailed,
}

/// Parameters for `ListInventorySupply`
#[allow(non_snake_case)]
#[derive(Debug, Default, Serialize, SerializeMwsParams)]
pub struct ListInventorySupplyParameters {
  // Optional API Parameters
  pub SellerSkus: Option<Vec<String>>,
  pub QueryStartDateTime: Option<DateTime<Utc>>,
  pub ResponseGroup: Option<ResponseGroup>,
  pub MarketplaceId: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Debug, Default, Serialize, FromXmlStream)]
pub struct ListInventorySupplyResponse {
  /// Indicates the specific marketplace to which the Inventory details apply.
  /// The element will only be included in the response if the corresponding
  /// request included a MarketplaceId. The value of the response MarketplaceId
  /// should match the corresponding request MarketplaceId.
  pub MarketplaceId: String,
  /// A structured list of items that are or soon will be available for fulfillment
  /// by the Amazon Fulfillment Network. Each item is either currently in the Amazon
  /// Fulfillment Network or is in an inbound shipment to an Amazon fulfillment center.
  /// SKU, ASIN, condition, quantity, and availability information is included with each item.
  pub InventorySupplyList: Vec<InventorySupply>,
  /// A generated string used to pass information to your next request. If NextToken is returned,
  /// pass the value of NextToken to ListInventorySupplyByNextToken. If NextToken is not returned,
  /// there is no more inventory availability information to return.
  pub NextToken: Option<String>,
}

response_envelope_type!(
  ListInventorySupplyResponseEnvelope<ListInventorySupplyResponse>,
  "ListInventorySupplyResponse",
  "ListInventorySupplyResult"
);

response_envelope_type!(
  ListInventorySupplyByNextTokenResponseEnvelope<ListInventorySupplyResponse>,
  "ListInventorySupplyByNextTokenResponse",
  "ListInventorySupplyByNextTokenResult"
);

/// The ListInventorySupply operation returns information about the availability of
/// inventory that a seller has in the Amazon Fulfillment Network and in current inbound shipments.
/// You can check the current availabilty status for your Amazon Fulfillment Network inventory as well
/// as discover when availability status changes.
///
/// This operation does not return availability information for inventory that is:
/// - Unsellable
/// - Bound to a customer order
///
/// [Documentation](http://docs.developer.amazonservices.com/en_CA/fba_inventory/FBAInventory_ListInventorySupply.html)
#[allow(non_snake_case)]
pub fn ListInventorySupply(
  client: &Client,
  parameters: ListInventorySupplyParameters,
) -> MwsResult<ListInventorySupplyResponse> {
  client
    .request_xml(
      Method::Post,
      PATH,
      VERSION,
      "ListInventorySupply",
      parameters,
    )
    .map(|e: ListInventorySupplyResponseEnvelope| e.into_inner())
    .map_err(|err| err.into())
}

/// Returns the next page of information about the availability of a seller's inventory using the NextToken parameter.
#[allow(non_snake_case)]
pub fn ListInventorySupplyByNextToken(
  client: &Client,
  next_token: String,
) -> MwsResult<ListInventorySupplyResponse> {
  let params = vec![("NextToken".to_string(), next_token)];
  client
    .request_xml(
      Method::Post,
      PATH,
      VERSION,
      "ListInventorySupplyByNextToken",
      params,
    )
    .map(|e: ListInventorySupplyByNextTokenResponseEnvelope| e.into_inner())
    .map_err(|err| err.into())
}

// #[cfg(test)]
// mod tests {
//   use dotenv::dotenv;
//   use super::*;
//   use super::super::client::get_test_client;

//   #[test]
//   fn test_list_inventory_supply() {
//     dotenv().ok();
//     let c = get_test_client();
//     let mut params = ListInventorySupplyParameters::default();
//     params.query_start_datetime = Some("2017-02-07T05:00:00Z".parse().expect("parse created_after"));
//     let res = ListInventorySupply(&c, params).expect("ListInventorySupply");
//     match res {
//       Response::Error(e) => panic!("request error: {:?}", e),
//       Response::Success(res) => {
//         println!("res = {:?}", res);
//       },
//     }
//   }

//   #[test]
//   fn test_list_inventory_supply_by_next_token() {
//     dotenv().ok();
//     let token = "H4sIAAAAAAAAADXPS3KCMAAA0Ks4TF25ACTGMlM7Q4oIYgoCIrqLBAifFJUg4unbTd8N3kfZXerJQO8rSdz7TJpwsZKmb1MbdI7xb3P0xuFmYoQsGRa1ku45cB2fWac7yjhESLu2a+KcVFkBqqBNQo7nGdR/NkFH5q+LyV7Nfpdn30oMS5FhpPXBJY3cs02KVAeR2jAyCEjF4lzUWRBVLd1tQ08YTH0+cnaz06/EmokWD8sDnSEtDkdQ187WrqxrwsVuQRJj+x6GPF1m1RP2WHkwnI5p9CptT1ZLumbtHLjdVWmaBbn5kel7mK9PrkmoEed6Tg4e94HjAt/UtQDn7V6r+hjGRYv+9itJ/vwFMFiuAicBAAA=_mEaNrCkReYPtTpElDaMcKeP_ATVPDKIKX0DER";
//     let c = get_test_client();
//     let res = ListInventorySupplyByNextToken(&c, token.to_owned()).expect("ListInventorySupplyByNextToken");
//     match res {
//       Response::Error(e) => panic!("request error: {:?}", e),
//       Response::Success(res) => {
//         println!("res = {:?}", res);
//       },
//     }
//   }
// }
