//! Amazon MWS Orders API - Version 2013-09-01
//!
//! [Documentation](http://docs.developer.amazonservices.com/en_US/orders-2013-09-01/Orders_Overview.html)

use chrono::{DateTime, Utc};
use client::{Client, Method, Response};
mod types;
pub use self::types::*;
use super::types::ToIso8601;
use result::MwsResult;
use xmlhelper::decode;

static PATH: &'static str = "/Orders/2013-09-01";
static VERSION: &'static str = "2013-09-01";

/// Parameters for `ListOrders`
#[derive(Debug, Default, Serialize)]
pub struct ListOrdersParameters {
  // Required API Parameters
  pub marketplace_id_list: Vec<String>,

  // Optional API Parameters
  pub created_after: Option<DateTime<Utc>>,
  pub created_before: Option<DateTime<Utc>>,
  pub last_updated_after: Option<DateTime<Utc>>,
  pub last_updated_before: Option<DateTime<Utc>>,
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
        result.push((
          format!("OrderStatus.Status.{}", i + 1),
          status.as_ref().to_string(),
        ));
      }
    }

    if let Some(list) = self.fulfillment_channel_list {
      for (i, channel) in list.iter().enumerate() {
        result.push((
          format!("FulfillmentChannel.Channel.{}", i + 1),
          channel.as_ref().to_string(),
        ));
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
        result.push((
          format!("PaymentMethod.Method.{}", i + 1),
          v.as_ref().to_string(),
        ));
      }
    }

    if let Some(list) = self.tfm_shipment_status {
      for (i, v) in list.iter().enumerate() {
        result.push((
          format!("TFMShipmentStatus.Status.{}", i + 1),
          v.as_ref().to_string(),
        ));
      }
    }

    if let Some(v) = self.max_results_per_page {
      result.push(("MaxResultsPerPage".to_string(), v.to_string()));
    }

    result
  }
}

#[derive(Debug, Default, Serialize)]
pub struct ListOrdersResponse {
  pub request_id: String,
  pub orders: Vec<Order>,
  pub last_updated_before: Option<DateTime<Utc>>,
  pub created_before: Option<DateTime<Utc>>,
  pub next_token: Option<String>,
}

impl<S: decode::XmlEventStream> decode::FromXmlStream<S> for ListOrdersResponse {
  fn from_xml(s: &mut S) -> MwsResult<ListOrdersResponse> {
    use self::decode::{all, characters, element, fold_elements, start_document};
    start_document(s)?;
    element(s, "ListOrdersResponse", |s| {
      fold_elements(s, ListOrdersResponse::default(), |s, response| {
        match s.local_name() {
          "ListOrdersResult" => fold_elements(s, (), |s, _| {
            match s.local_name() {
              "Orders" => {
                response.orders = all(s, |s| Order::from_xml(s))?;
              }
              "CreatedBefore" => {
                response.created_before = Some(characters(s)?);
              }
              "LastUpdatedBefore" => {
                response.last_updated_before = Some(characters(s)?);
              }
              "NextToken" => {
                response.next_token = Some(characters(s)?);
              }
              _ => {}
            }
            Ok(())
          }),
          "ResponseMetadata" => {
            response.request_id = element(s, "RequestId", |s| characters(s))?;
            Ok(())
          }
          _ => Ok(()),
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
#[allow(non_snake_case)]
pub fn ListOrders(
  client: &Client,
  parameters: ListOrdersParameters,
) -> MwsResult<Response<ListOrdersResponse>> {
  client
    .request_xml(Method::Post, PATH, VERSION, "ListOrders", parameters)
    .map_err(|err| err.into())
}

#[derive(Debug, Default, Serialize)]
pub struct ListOrdersByNextTokenResponse {
  pub request_id: String,
  pub orders: Vec<Order>,
  pub last_updated_before: Option<DateTime<Utc>>,
  pub next_token: Option<String>,
}

impl<S: decode::XmlEventStream> decode::FromXmlStream<S> for ListOrdersByNextTokenResponse {
  fn from_xml(s: &mut S) -> MwsResult<ListOrdersByNextTokenResponse> {
    use self::decode::{all, characters, element, fold_elements, start_document};
    start_document(s)?;
    element(s, "ListOrdersByNextTokenResponse", |s| {
      fold_elements(
        s,
        ListOrdersByNextTokenResponse::default(),
        |s, response| match s.local_name() {
          "ListOrdersByNextTokenResult" => fold_elements(s, (), |s, _| {
            match s.local_name() {
              "Orders" => {
                response.orders = all(s, |s| Order::from_xml(s))?;
              }
              "LastUpdatedBefore" => {
                response.last_updated_before = Some(characters(s)?);
              }
              "NextToken" => {
                response.next_token = Some(characters(s)?);
              }
              _ => {}
            }
            Ok(())
          }),
          "ResponseMetadata" => {
            response.request_id = element(s, "RequestId", |s| characters(s))?;
            Ok(())
          }
          _ => Ok(()),
        },
      )
    })
  }
}

/// Returns the next page of orders using the NextToken parameter.
///
/// The ListOrdersByNextToken operation returns the next page of orders using the NextToken value that was returned
/// by your previous request to either ListOrders or ListOrdersByNextToken.
/// If NextToken is not returned, there are no more pages to return.
#[allow(non_snake_case)]
pub fn ListOrdersByNextToken(
  client: &Client,
  next_token: String,
) -> MwsResult<Response<ListOrdersByNextTokenResponse>> {
  let params = vec![("NextToken".to_string(), next_token)];
  client
    .request_xml(Method::Post, PATH, VERSION, "ListOrdersByNextToken", params)
    .map_err(|err| err.into())
}

#[derive(Debug, Default, Serialize)]
pub struct ListOrderItemsResponse {
  pub request_id: String,
  pub items: Vec<OrderItem>,
  pub amazon_order_id: String,
  pub next_token: Option<String>,
}

impl<S: decode::XmlEventStream> decode::FromXmlStream<S> for ListOrderItemsResponse {
  fn from_xml(s: &mut S) -> MwsResult<ListOrderItemsResponse> {
    use self::decode::{all, characters, element, fold_elements, start_document};
    start_document(s)?;
    element(s, "ListOrderItemsResponse", |s| {
      fold_elements(
        s,
        ListOrderItemsResponse::default(),
        |s, response| match s.local_name() {
          "ListOrderItemsResult" => fold_elements(s, (), |s, _| {
            match s.local_name() {
              "OrderItems" => {
                response.items = all(s, |s| OrderItem::from_xml(s))?;
              }
              "AmazonOrderId" => {
                response.amazon_order_id = characters(s)?;
              }
              "NextToken" => {
                response.next_token = Some(characters(s)?);
              }
              _ => {}
            }
            Ok(())
          }),
          "ResponseMetadata" => {
            response.request_id = element(s, "RequestId", |s| characters(s))?;
            Ok(())
          }
          _ => Ok(()),
        },
      )
    })
  }
}

/// Returns order items based on the AmazonOrderId that you specify.
#[allow(non_snake_case)]
pub fn ListOrderItems(
  client: &Client,
  amazon_order_id: String,
) -> MwsResult<Response<ListOrderItemsResponse>> {
  let params = vec![("AmazonOrderId".to_string(), amazon_order_id)];
  client
    .request_xml(Method::Post, PATH, VERSION, "ListOrderItems", params)
    .map_err(|err| err.into())
}

#[derive(Debug, Default, Serialize)]
pub struct ListOrderItemsByNextTokenResponse {
  pub request_id: String,
  pub items: Vec<OrderItem>,
  pub amazon_order_id: String,
  pub next_token: Option<String>,
}

/// Returns the next page of order items using the NextToken parameter.
#[allow(non_snake_case)]
pub fn ListOrderItemsByNextToken(
  client: &Client,
  next_token: String,
) -> MwsResult<Response<ListOrderItemsByNextTokenResponse>> {
  let params = vec![("NextToken".to_string(), next_token)];
  client
    .request_xml(
      Method::Post,
      PATH,
      VERSION,
      "ListOrderItemsByNextToken",
      params,
    ).map_err(|err| err.into())
}

impl<S: decode::XmlEventStream> decode::FromXmlStream<S> for ListOrderItemsByNextTokenResponse {
  fn from_xml(s: &mut S) -> MwsResult<ListOrderItemsByNextTokenResponse> {
    use self::decode::{all, characters, element, fold_elements, start_document};
    start_document(s)?;
    element(s, "ListOrderItemsByNextTokenResponse", |s| {
      fold_elements(
        s,
        ListOrderItemsByNextTokenResponse::default(),
        |s, response| match s.local_name() {
          "ListOrderItemsByNextTokenResult" => fold_elements(s, (), |s, _| {
            match s.local_name() {
              "OrderItems" => {
                response.items = all(s, |s| OrderItem::from_xml(s))?;
              }
              "AmazonOrderId" => {
                response.amazon_order_id = characters(s)?;
              }
              "NextToken" => {
                response.next_token = Some(characters(s)?);
              }
              _ => {}
            }
            Ok(())
          }),
          "ResponseMetadata" => {
            response.request_id = element(s, "RequestId", |s| characters(s))?;
            Ok(())
          }
          _ => Ok(()),
        },
      )
    })
  }
}

// #[cfg(test)]
// mod tests {
//   use dotenv::dotenv;
//   use super::*;
//   use super::super::client::get_test_client;

//   #[test]
//   fn test_list_orders() {
//     dotenv().ok();
//     let c = get_test_client();
//     let mut params = ListOrdersParameters::default();
//     params.marketplace_id_list.push("ATVPDKIKX0DER".to_string());
//     params.created_after = Some("2016-11-01T04:00:00Z".parse().expect("parse created_after"));
//     params.max_results_per_page = Some(1);
//     let res = ListOrders(&c, params).expect("ListOrders");
//     match res {
//       Response::Error(e) => panic!("request error: {:?}", e),
//       _ => {},
//     }
//   }

//   #[test]
//   fn test_list_order_items() {
//     dotenv().ok();
//     let c = get_test_client();
//     let res = ListOrderItems(&c, "112-8095165-5463447".to_string()).expect("ListOrderItems");
//     match res {
//       Response::Error(e) => panic!("request error: {:?}", e),
//       Response::Success(ListOrderItemsResponse { amazon_order_id, items, .. }) => {
//         assert_eq!(items.len(), 1);
//         assert_eq!(amazon_order_id, "112-8095165-5463447");
//         println!("{:?}", items[0]);
//       },
//     }
//   }
// }
