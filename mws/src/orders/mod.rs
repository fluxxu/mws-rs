//! Amazon MWS Orders API - Version 2013-09-01
//!
//! [Documentation](http://docs.developer.amazonservices.com/en_US/orders-2013-09-01/Orders_Overview.html)

use chrono::{DateTime, Utc};
use client::{Client, Method};
mod types;
pub use self::types::*;
use result::MwsResult;

static PATH: &'static str = "/Orders/2013-09-01";
static VERSION: &'static str = "2013-09-01";

/// Parameters for `ListOrders`
#[allow(non_snake_case)]
#[derive(Debug, Default, Serialize, SerializeMwsParams)]
pub struct ListOrdersParameters {
  // Required API Parameters
  #[mws_param(list_item_type_name = "Id")]
  pub MarketplaceId: Vec<String>,

  // Optional API Parameters
  pub CreatedAfter: Option<DateTime<Utc>>,
  pub CreatedBefore: Option<DateTime<Utc>>,
  pub LastUpdatedAfter: Option<DateTime<Utc>>,
  pub LastUpdatedBefore: Option<DateTime<Utc>>,
  #[mws_param(list_item_type_name = "Status")]
  pub OrderStatus: Option<Vec<OrderStatus>>,
  #[mws_param(list_item_type_name = "Channel")]
  pub FulfillmentChannel: Option<Vec<FulfillmentChannel>>,
  pub SellerOrderId: Option<String>,
  pub BuyerEmail: Option<String>,
  #[mws_param(list_item_type_name = "Method")]
  pub PaymentMethod: Option<Vec<PaymentMethod>>,
  #[mws_param(list_item_type_name = "Status")]
  pub TFMShipmentStatus: Option<Vec<TFMShipmentStatus>>,
  pub MaxResultsPerPage: Option<i32>,
}

#[allow(non_snake_case)]
#[derive(Debug, Default, Serialize, FromXmlStream)]
pub struct ListOrdersResponse {
  pub Orders: Vec<Order>,
  pub LastUpdatedBefore: Option<DateTime<Utc>>,
  pub CreatedBefore: Option<DateTime<Utc>>,
  pub NextToken: Option<String>,
}

response_envelope_type!(
  ListOrdersEnvelope<ListOrdersResponse>,
  "ListOrdersResponse",
  "ListOrdersResult"
);

response_envelope_type!(
  ListOrdersByNextTokenEnvelope<ListOrdersResponse>,
  "ListOrdersByNextTokenResponse",
  "ListOrdersByNextTokenResult"
);

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
) -> MwsResult<ListOrdersResponse> {
  client
    .request_xml(Method::Post, PATH, VERSION, "ListOrders", parameters)
    .map(|e: ListOrdersEnvelope| e.into_inner())
    .map_err(|err| err.into())
}

/// Returns the next page of orders using the NextToken parameter.
///
/// The ListOrdersByNextToken operation returns the next page of orders using the NextToken value that was returned
/// by your previous request to either ListOrders or ListOrdersByNextToken.
/// If NextToken is not returned, there are no more pages to return.
#[allow(non_snake_case)]
pub fn ListOrdersByNextToken(client: &Client, next_token: String) -> MwsResult<ListOrdersResponse> {
  let params = vec![("NextToken".to_string(), next_token)];
  client
    .request_xml(Method::Post, PATH, VERSION, "ListOrdersByNextToken", params)
    .map(|e: ListOrdersByNextTokenEnvelope| e.into_inner())
    .map_err(|err| err.into())
}

#[allow(non_snake_case)]
#[derive(Debug, Default, Serialize, FromXmlStream)]
pub struct ListOrderItemsResponse {
  pub OrderItems: Vec<OrderItem>,
  pub AmazonOrderId: String,
  pub NextToken: Option<String>,
}

response_envelope_type!(
  ListOrderItemsEnvelope<ListOrderItemsResponse>,
  "ListOrderItemsResponse",
  "ListOrderItemsResult"
);

response_envelope_type!(
  ListOrderItemsByNextTokenEnvelope<ListOrderItemsResponse>,
  "ListOrderItemsByNextTokenResponse",
  "ListOrderItemsByNextTokenResult"
);

/// Returns order items based on the AmazonOrderId that you specify.
#[allow(non_snake_case)]
pub fn ListOrderItems(
  client: &Client,
  amazon_order_id: String,
) -> MwsResult<ListOrderItemsResponse> {
  let params = vec![("AmazonOrderId".to_string(), amazon_order_id)];
  client
    .request_xml(Method::Post, PATH, VERSION, "ListOrderItems", params)
    .map(|e: ListOrderItemsEnvelope| e.into_inner())
    .map_err(|err| err.into())
}

/// Returns the next page of order items using the NextToken parameter.
#[allow(non_snake_case)]
pub fn ListOrderItemsByNextToken(
  client: &Client,
  next_token: String,
) -> MwsResult<ListOrderItemsResponse> {
  let params = vec![("NextToken".to_string(), next_token)];
  client
    .request_xml(
      Method::Post,
      PATH,
      VERSION,
      "ListOrderItemsByNextToken",
      params,
    )
    .map(|e: ListOrderItemsByNextTokenEnvelope| e.into_inner())
    .map_err(|err| err.into())
}

#[cfg(test)]
mod tests {
  use super::super::client::get_test_client;
  use super::*;
  use dotenv::dotenv;

  #[test]
  #[ignore]
  fn test_list_orders() {
    dotenv().ok();
    let c = get_test_client();
    let mut params = ListOrdersParameters::default();
    params.MarketplaceId.push("ATVPDKIKX0DER".to_string());
    params.CreatedAfter = Some("2016-11-01T04:00:00Z".parse().expect("parse created_after"));
    params.MaxResultsPerPage = Some(1);
    let res = ListOrders(&c, params).expect("ListOrders");
    println!("{:#?}", res)
  }

  #[test]
  #[ignore]
  fn test_list_order_items() {
    dotenv().ok();
    let c = get_test_client();
    let res = ListOrderItems(&c, "112-8095165-5463447".to_string()).expect("ListOrderItems");
    println!("{:#?}", res)
  }
}
