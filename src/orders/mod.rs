//! Amazon MWS Orders API - Version 2013-09-01
//!
//! [Documentation](http://docs.developer.amazonservices.com/en_US/orders-2013-09-01/Orders_Overview.html)

use chrono::{DateTime, UTC};
use client::{Client, Method, Response};
mod types;
pub use self::types::{Order, OrderStatus, FulfillmentChannel, PaymentMethod, TFMShipmentStatus};
use xmlhelper::decode;
use super::types::ToIso8601;

error_chain! {
  links {
    Client(super::client::Error, super::client::ErrorKind);
    Decode(decode::Error, decode::ErrorKind);
  }
}

static PATH: &'static str = "/Orders";
static VERSION: &'static str = "2013-09-01";

/// Parameters for `list_orders`
#[derive(Debug, Default)]
pub struct ListOrdersParameters {
  // Required API Parameters
  pub marketplace_id_list: Vec<String>,

  // Optional API Parameters
  pub created_after: Option<DateTime<UTC>>,
  pub created_before: Option<DateTime<UTC>>,
  pub last_updated_after: Option<DateTime<UTC>>,
  pub last_updated_before: Option<DateTime<UTC>>,
  pub order_status_list: Option<Vec<OrderStatus>>,
  pub fulfillment_channel_list: Option<Vec<FulfillmentChannel>>,
  pub seller_order_id: Option<String>,
  pub buyer_email: Option<String>,
  pub payment_method_list: Option<Vec<PaymentMethod>>,
  pub tfm_shipment_status: Option<Vec<TFMShipmentStatus>>,
  pub max_results_per_page: Option<i32>,
}

impl Into<Vec<(String, String)>> for ListOrdersParameters {
  fn into(self) -> Vec<(String, String)> {
    let mut result = vec![];
    for (i, id) in self.marketplace_id_list.into_iter().enumerate() {
      result.push((format!("MarketplaceId.Id.{}", i + 1), id));
    }

    if let Some(date) = self.created_after {
      result.push(("CreatedAfter".to_string(), date.to_iso8601()));
    }

    if let Some(date) = self.created_before {
      result.push(("CreatedBefore".to_string(), date.to_iso8601()));
    }

    if let Some(date) = self.last_updated_after {
      result.push(("LastUpdatedAfter".to_string(), date.to_iso8601()));
    }

    if let Some(date) = self.last_updated_before {
      result.push(("LastUpdatedBefore".to_string(), date.to_iso8601()));
    }

    if let Some(list) = self.order_status_list {
      for (i, status) in list.iter().enumerate() {
        result.push((format!("OrderStatus.Status.{}", i + 1), status.as_ref().to_string()));
      }
    }

    if let Some(list) = self.fulfillment_channel_list {
      for (i, channel) in list.iter().enumerate() {
        result.push((format!("FulfillmentChannel.Channel.{}", i + 1), channel.as_ref().to_string()));
      }
    }

    if let Some(id) = self.seller_order_id {
      result.push(("SellerOrderId".to_string(), id));
    }

    if let Some(email) = self.buyer_email {
      result.push(("BuyerEmail".to_string(), email));
    }

    if let Some(list) = self.payment_method_list {
      for (i, v) in list.iter().enumerate() {
        result.push((format!("PaymentMethod.Method.{}", i + 1), v.as_ref().to_string()));
      }
    }

    if let Some(list) = self.tfm_shipment_status {
      for (i, v) in list.iter().enumerate() {
        result.push((format!("TFMShipmentStatus.Status.{}", i + 1), v.as_ref().to_string()));
      }
    }

    if let Some(v) = self.max_results_per_page {
      result.push(("MaxResultsPerPage".to_string(), v.to_string()));
    }

    result
  }
}

#[derive(Debug, Default)]
pub struct ListOrdersResponse {
  pub request_id: String,
  pub orders: Vec<Order>,
  pub last_updated_before: Option<DateTime<UTC>>,
  pub created_before: Option<DateTime<UTC>>,
  pub next_token: Option<String>,
}

impl<S: decode::XmlEventStream> decode::FromXMLStream<S> for ListOrdersResponse {
  fn from_xml(s: &mut S) -> decode::Result<ListOrdersResponse> {
    use self::decode::{start_document, element, fold_elements, all, characters};
    start_document(s)?;
    element(s, "ListOrdersResponse", |s| {
      fold_elements(s, ListOrdersResponse::default(), |s, response| {
        match s.local_name() {
          "ListOrdersResult" => {
            fold_elements(s, (), |s, _| {
              match s.local_name() {
                "Orders" => {
                  response.orders = all(s, |s| Order::from_xml(s))?;
                },
                "CreatedBefore" => {
                  response.created_before = Some(characters(s)?);
                },
                "LastUpdatedBefore" => {
                  response.last_updated_before = Some(characters(s)?);
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

/// Returns orders created or updated during a time frame that you specify.
///
/// The ListOrders operation returns a list of orders created or updated during a time frame that you specify. 
/// You define that time frame using the CreatedAfter parameter or the LastUpdatedAfter parameter. 
/// You must use one of these parameters, but not both. You can also apply a range of filtering criteria to narrow the list of orders that is returned. 
/// The ListOrders operation includes order information for each order returned, including AmazonOrderId, OrderStatus, FulfillmentChannel, and LastUpdateDate.
///
/// [Documentation](http://docs.developer.amazonservices.com/en_US/orders-2013-09-01/Orders_ListOrders.html)
pub fn list_orders(client: &Client, parameters: ListOrdersParameters) -> Result<Response<ListOrdersResponse>> {
  client.request(Method::Post, PATH, VERSION, "ListOrders", parameters).map_err(|err| err.into())
}


/// Returns the next page of orders using the NextToken parameter.
///
/// The ListOrdersByNextToken operation returns the next page of orders using the NextToken value that was returned
/// by your previous request to either ListOrders or ListOrdersByNextToken. 
/// If NextToken is not returned, there are no more pages to return.
pub fn list_orders_by_next_token() { unimplemented!() }

pub fn get_order() { unimplemented!() }

#[cfg(test)]
mod tests {
  use dotenv::dotenv;
  use super::*;
  use super::super::client::get_test_client;

  #[test]
  fn test_list_orders() {
    dotenv().ok();
    let c = get_test_client();
    let mut params = ListOrdersParameters::default();
    params.marketplace_id_list.push("ATVPDKIKX0DER".to_string());
    params.created_after = Some("2016-11-01T04:00:00Z".parse().expect("parse created_after"));
    params.max_results_per_page = Some(1);
    let res = list_orders(&c, params).expect("list_orders");
    match res {
      Response::Error(e) => panic!("request error: {:?}", e),
      _ => {},
    }
  }
}