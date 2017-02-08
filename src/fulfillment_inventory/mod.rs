//! Amazon MWS Fulfillment Inventory API - Version 2010-10-01
//!
//! [Documentation](http://docs.developer.amazonservices.com/en_CA/fba_inventory/FBAInventory_Overview.html)

use chrono::{DateTime, UTC};
use client::{Client, Method, Response};
mod types;
pub use self::types::{InventorySupply};
use xmlhelper::decode;
use super::types::ToIso8601;

error_chain! {
  links {
    Client(super::client::Error, super::client::ErrorKind);
    Decode(decode::Error, decode::ErrorKind);
  }
}

static PATH: &'static str = "/FulfillmentInventory/2010-10-01";
static VERSION: &'static str = "2010-10-01";

#[derive(Debug)]
pub enum ResponseGroup {
  Basic,
  Detailed,
}

/// Parameters for `ListInventorySupply`
#[derive(Debug, Default)]
pub struct ListInventorySupplyParameters {
  // Optional API Parameters
  pub seller_sku_list: Vec<String>,
  pub query_start_datetime: Option<DateTime<UTC>>,
  pub response_group: Option<ResponseGroup>,
  pub marketplace_id: Option<String>,  
}

impl Into<Vec<(String, String)>> for ListInventorySupplyParameters {
  fn into(self) -> Vec<(String, String)> {
    let mut result = vec![];
    for (i, id) in self.seller_sku_list.into_iter().enumerate() {
      result.push((format!("SellerSkus.member.{}", i + 1), id));
    }

    if let Some(date) = self.query_start_datetime {
      result.push(("QueryStartDateTime".to_string(), date.to_iso8601()));
    }

    if let Some(group) = self.response_group {
      result.push(("ResponseGroup".to_string(), match group {
        ResponseGroup::Basic => "Basic".to_owned(),
        ResponseGroup::Detailed => "Detailed".to_owned(),
      }));
    }

    if let Some(id) = self.marketplace_id {
      result.push(("MarketplaceId".to_string(), id));
    }

    result
  }
}

#[derive(Debug, Default)]
pub struct ListInventorySupplyResponse {
  pub request_id: String,
  /// Indicates the specific marketplace to which the Inventory details apply. 
  /// The element will only be included in the response if the corresponding 
  /// request included a MarketplaceId. The value of the response MarketplaceId 
  /// should match the corresponding request MarketplaceId.
  pub marketplace_id: String,
  /// A structured list of items that are or soon will be available for fulfillment
  /// by the Amazon Fulfillment Network. Each item is either currently in the Amazon
  /// Fulfillment Network or is in an inbound shipment to an Amazon fulfillment center. 
  /// SKU, ASIN, condition, quantity, and availability information is included with each item.
  pub inventory_supply_list: Vec<InventorySupply>,
  /// A generated string used to pass information to your next request. If NextToken is returned,
  /// pass the value of NextToken to ListInventorySupplyByNextToken. If NextToken is not returned, 
  /// there is no more inventory availability information to return.
  pub next_token: Option<String>,
}

impl<S: decode::XmlEventStream> decode::FromXMLStream<S> for ListInventorySupplyResponse {
  fn from_xml(s: &mut S) -> decode::Result<ListInventorySupplyResponse> {
    use self::decode::{start_document, element, fold_elements, characters};
    start_document(s)?;
    element(s, "ListInventorySupplyResponse", |s| {
      fold_elements(s, ListInventorySupplyResponse::default(), |s, response| {
        match s.local_name() {
          "ListInventorySupplyResult" => {
            fold_elements(s, (), |s, _| {
              match s.local_name() {
                "InventorySupplyList" => {
                  response.inventory_supply_list = fold_elements(s, vec![], |s, list| {
                    list.push(InventorySupply::from_xml(s)?);
                    Ok(())
                  })?;
                },
                "MarketplaceId" => {
                  response.marketplace_id = characters(s)?;
                },
                "NextToken" => {
                  response.next_token = Some(characters(s)?);
                },
                _ => {},
              }
              Ok(())
            })
          },
          "ResponseMetadata" => {
            response.request_id = element(s, "RequestId", |s| {
              characters(s)
            })?;
            Ok(())
          },
          _ => { Ok(()) }
        }
      })
    })
  }
}

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
pub fn ListInventorySupply(client: &Client, parameters: ListInventorySupplyParameters) -> Result<Response<ListInventorySupplyResponse>> {
  client.request_xml(Method::Post, PATH, VERSION, "ListInventorySupply", parameters).map_err(|err| err.into())
}

#[derive(Debug, Default)]
pub struct ListInventorySupplyByNextTokenResponse {
  pub request_id: String,
  /// Indicates the specific marketplace to which the Inventory details apply. 
  /// The element will only be included in the response if the corresponding 
  /// request included a MarketplaceId. The value of the response MarketplaceId 
  /// should match the corresponding request MarketplaceId.
  pub marketplace_id: String,
  /// A structured list of items that are or soon will be available for fulfillment
  /// by the Amazon Fulfillment Network. Each item is either currently in the Amazon
  /// Fulfillment Network or is in an inbound shipment to an Amazon fulfillment center. 
  /// SKU, ASIN, condition, quantity, and availability information is included with each item.
  pub inventory_supply_list: Vec<InventorySupply>,
  /// A generated string used to pass information to your next request. If NextToken is returned,
  /// pass the value of NextToken to ListInventorySupplyByNextToken. If NextToken is not returned, 
  /// there is no more inventory availability information to return.
  pub next_token: Option<String>,
}

impl<S: decode::XmlEventStream> decode::FromXMLStream<S> for ListInventorySupplyByNextTokenResponse {
  fn from_xml(s: &mut S) -> decode::Result<ListInventorySupplyByNextTokenResponse> {
    use self::decode::{start_document, element, fold_elements, characters};
    start_document(s)?;
    element(s, "ListInventorySupplyByNextTokenResponse", |s| {
      fold_elements(s, ListInventorySupplyByNextTokenResponse::default(), |s, response| {
        match s.local_name() {
          "ListInventorySupplyByNextTokenResult" => {
            fold_elements(s, (), |s, _| {
              match s.local_name() {
                "InventorySupplyList" => {
                  response.inventory_supply_list = fold_elements(s, vec![], |s, list| {
                    list.push(InventorySupply::from_xml(s)?);
                    Ok(())
                  })?;
                },
                "MarketplaceId" => {
                  response.marketplace_id = characters(s)?;
                },
                "NextToken" => {
                  response.next_token = Some(characters(s)?);
                },
                _ => {},
              }
              Ok(())
            })
          },
          "ResponseMetadata" => {
            response.request_id = element(s, "RequestId", |s| {
              characters(s)
            })?;
            Ok(())
          },
          _ => { Ok(()) }
        }
      })
    })
  }
}

/// Returns the next page of information about the availability of a seller's inventory using the NextToken parameter.
#[allow(non_snake_case)]
pub fn ListInventorySupplyByNextToken(client: &Client, next_token: String) -> Result<Response<ListInventorySupplyByNextTokenResponse>> {
  let params = vec![
    ("NextToken".to_string(), next_token)
  ]; 
  client.request_xml(Method::Post, PATH, VERSION, "ListInventorySupplyByNextToken", params).map_err(|err| err.into())
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