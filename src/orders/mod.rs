//! Amazon MWS Orders API - Version 2013-09-01
//!
//! [Documentation](http://docs.developer.amazonservices.com/en_US/orders-2013-09-01/Orders_Overview.html)

use chrono::{DateTime, UTC};
use client::{Client, Method};
mod types;
pub use self::types::{Order};

error_chain! {
  links {
    Client(super::client::Error, super::client::ErrorKind);
  }
}

static PATH: &'static str = "/Orders/2013-09-01";

/// Parameters for `list_orders`
#[derive(Debug)]
pub struct ListOrdersParameters {
  // Required API Parameters
  pub marketplace_id_list: Vec<String>,

  // Optional API Parameters
  pub created_after: Option<DateTime<UTC>>,
  pub created_before: Option<DateTime<UTC>>,
  pub last_updated_after: Option<DateTime<UTC>>,
  pub last_updated_before: Option<DateTime<UTC>>,
  pub order_status_list: Option<Vec<String>>,
  pub fulfillment_channel_list: Option<Vec<String>>,
  pub seller_order_id: Option<String>,
  pub buyer_email: Option<String>,
  pub payment_method_list: Option<Vec<String>>,
  pub tfm_shipment_status: Option<Vec<String>>,
  pub max_results_per_page: Option<i32>,
}

impl Into<Vec<(String, String)>> for ListOrdersParameters {
  fn into(self) -> Vec<(String, String)> {
    let mut result = vec![];
    for (i, id) in self.marketplace_id_list.iter().enumerate() {

    }
    result
  }
}

#[derive(Debug, Default)]
pub struct ListOrdersResponse {
  pub request_id: String,
  pub orders: Vec<Order>,
  pub created_before: Option<DateTime<UTC>>,
  pub next_token: Option<String>,
}

/// Returns orders created or updated during a time frame that you specify.
///
/// The ListOrders operation returns a list of orders created or updated during a time frame that you specify. 
/// You define that time frame using the CreatedAfter parameter or the LastUpdatedAfter parameter. 
/// You must use one of these parameters, but not both. You can also apply a range of filtering criteria to narrow the list of orders that is returned. 
/// The ListOrders operation includes order information for each order returned, including AmazonOrderId, OrderStatus, FulfillmentChannel, and LastUpdateDate.
///
/// [Documentation](http://docs.developer.amazonservices.com/en_US/orders-2013-09-01/Orders_ListOrders.html)
pub fn list_orders(client: &Client, parameters: ListOrdersParameters) -> Result<ListOrdersResponse> {
  client.request(Method::Get, PATH, parameters).map_err(|err| err.into())
}

/// Returns the next page of orders using the NextToken parameter.
///
/// The ListOrdersByNextToken operation returns the next page of orders using the NextToken value that was returned
/// by your previous request to either ListOrders or ListOrdersByNextToken. 
/// If NextToken is not returned, there are no more pages to return.
pub fn list_orders_by_next_token() { unimplemented!() }

pub fn get_order() { unimplemented!() }