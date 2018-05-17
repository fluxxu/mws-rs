//! Fulfillment Outbound Shipment API - Version 2010-10-01
//!
//! [Documentation](https://docs.developer.amazonservices.com/en_US/fba_outbound/FBAOutbound_Overview.html)

use chrono::{DateTime, Utc};
use client::{Client, Method, Response};
mod types;
pub use self::types::*;
use super::types::ToIso8601;
use xmlhelper::decode;

error_chain! {
  links {
    Client(super::client::Error, super::client::ErrorKind);
    Decode(decode::Error, decode::ErrorKind);
  }
}

static PATH: &'static str = "/FulfillmentOutboundShipment/2010-10-01";
static VERSION: &'static str = "2010-10-01";

#[derive(Debug, Default)]
pub struct ListAllFulfillmentOrdersResponse {
  pub request_id: String,
  pub fulfillment_orders: Vec<FulfillmentOrder>,
  pub next_token: Option<String>,
}

impl<S: decode::XmlEventStream> decode::FromXMLStream<S> for ListAllFulfillmentOrdersResponse {
  fn from_xml(s: &mut S) -> decode::Result<ListAllFulfillmentOrdersResponse> {
    use self::decode::{characters, element, fold_elements, start_document};
    start_document(s)?;
    element(
      s,
      vec![
        "ListAllFulfillmentOrdersResponse",
        "ListAllFulfillmentOrdersByNextTokenResponse",
      ],
      |s| {
        fold_elements(
          s,
          ListAllFulfillmentOrdersResponse::default(),
          |s, response| match s.local_name() {
            "ListAllFulfillmentOrdersResult" | "ListAllFulfillmentOrdersByNextTokenResult" => {
              fold_elements(s, (), |s, _| {
                match s.local_name() {
                  "FulfillmentOrders" => {
                    response.fulfillment_orders = fold_elements(s, vec![], |s, list| {
                      list.push(FulfillmentOrder::from_xml(s)?);
                      Ok(())
                    })?;
                  }
                  "NextToken" => {
                    response.next_token = Some(characters(s)?);
                  }
                  _ => {}
                }
                Ok(())
              })
            }
            "ResponseMetadata" => {
              response.request_id = element(s, "RequestId", |s| characters(s))?;
              Ok(())
            }
            _ => Ok(()),
          },
        )
      },
    )
  }
}

/// Returns a list of fulfillment orders fulfilled after (or at) a specified date.
///
/// [Documentation](http://docs.developer.amazonservices.com/en_US/orders-2013-09-01/Orders_ListOrders.html)
#[allow(non_snake_case)]
pub fn ListAllFulfillmentOrders(
  client: &Client,
  query_start_date_time: DateTime<Utc>,
) -> Result<Response<ListAllFulfillmentOrdersResponse>> {
  client
    .request_xml(
      Method::Post,
      PATH,
      VERSION,
      "ListAllFulfillmentOrders",
      vec![(
        "QueryStartDateTime".to_string(),
        query_start_date_time.to_iso8601(),
      )],
    )
    .map_err(|err| err.into())
}

/// Returns the next page of fulfillment orders using the NextToken parameter.
///
/// [Documentation](https://docs.developer.amazonservices.com/en_US/fba_outbound/FBAOutbound_ListAllFulfillmentOrdersByNextToken.html)
#[allow(non_snake_case)]
pub fn ListAllFulfillmentOrdersByNextToken(
  client: &Client,
  next_token: String,
) -> Result<Response<ListAllFulfillmentOrdersResponse>> {
  let params = vec![("NextToken".to_string(), next_token)];
  client
    .request_xml(
      Method::Post,
      PATH,
      VERSION,
      "ListAllFulfillmentOrdersByNextToken",
      params,
    )
    .map_err(|err| err.into())
}

#[derive(Debug, Default)]
#[allow(non_snake_case)]
pub struct GetFulfillmentOrderResponse {
  pub fulfillment_shipments: Vec<FulfillmentShipment>,
  pub return_item_list: Vec<ReturnItemList>,
  pub return_authorization_list: Vec<ReturnAuthorizationList>,
  pub fulfillment_order: FulfillmentOrder,
  pub fulfillment_order_items: Vec<FulfillmentOrderItem>,
  pub request_id: String,
}

impl<S: decode::XmlEventStream> decode::FromXMLStream<S> for GetFulfillmentOrderResponse {
  fn from_xml(s: &mut S) -> decode::Result<GetFulfillmentOrderResponse> {
    use self::decode::{characters, element, fold_elements, start_document};
    start_document(s)?;
    element(s, "GetFulfillmentOrderResponse", |s| {
      fold_elements(
        s,
        GetFulfillmentOrderResponse::default(),
        |s, response| match s.local_name() {
          "GetFulfillmentOrderResult" => fold_elements(s, (), |s, _| {
            match s.local_name() {
              "FulfillmentShipment" => {
                response.fulfillment_shipments = fold_elements(s, vec![], |s, list| {
                  list.push(FulfillmentShipment::from_xml(s)?);
                  Ok(())
                })?;
              }
              "FulfillmentOrder" => response.fulfillment_order = FulfillmentOrder::from_xml(s)?,
              "FulfillmentOrderItem" => {
                response.fulfillment_order_items = fold_elements(s, vec![], |s, list| {
                  list.push(FulfillmentOrderItem::from_xml(s)?);
                  Ok(())
                })?
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

/// Returns a fulfillment order based on a specified SellerFulfillmentOrderId.
///
/// [Documentation](http://docs.developer.amazonservices.com/en_CA/fba_outbound/FBAOutbound_GetFulfillmentOrder.html)
#[allow(non_snake_case)]
pub fn GetFulfillmentOrder(
  client: &Client,
  seller_fulfillment_order_id: String,
) -> Result<Response<GetFulfillmentOrderResponse>> {
  let params = vec![(
    "SellerFulfillmentOrderId".to_string(),
    seller_fulfillment_order_id,
  )];
  client
    .request_xml(Method::Post, PATH, VERSION, "GetFulfillmentOrder", params)
    .map_err(|err| err.into())
}

#[derive(Debug, Default)]
#[allow(non_snake_case)]
pub struct GetPackageTrackingDetailsResponse {
  pub details: PackageTrackingDetails,
  pub request_id: String,
}

impl<S: decode::XmlEventStream> decode::FromXMLStream<S> for GetPackageTrackingDetailsResponse {
  fn from_xml(s: &mut S) -> decode::Result<GetPackageTrackingDetailsResponse> {
    use self::decode::{characters, element, fold_elements, start_document};
    start_document(s)?;
    element(s, "GetPackageTrackingDetailsResponse", |s| {
      fold_elements(
        s,
        GetPackageTrackingDetailsResponse::default(),
        |s, response| match s.local_name() {
          "GetPackageTrackingDetailsResult" => {
            response.details = PackageTrackingDetails::from_xml(s)?;
            Ok(())
          }
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

/// Returns delivery tracking information for a package in an outbound shipment for a Multi-Channel Fulfillment order.
///
/// [Documentation](http://docs.developer.amazonservices.com/en_US/fba_outbound/FBAOutbound_GetPackageTrackingDetails.html)
#[allow(non_snake_case)]
pub fn GetPackageTrackingDetails(
  client: &Client,
  package_number: &str,
) -> Result<Response<GetPackageTrackingDetailsResponse>> {
  let params = vec![("PackageNumber".to_string(), package_number.to_owned())];
  client
    .request_xml(
      Method::Post,
      PATH,
      VERSION,
      "GetPackageTrackingDetails",
      params,
    )
    .map_err(|err| err.into())
}

/// Item information for a fulfillment order preview.
#[allow(non_snake_case)]
#[derive(Debug, Default)]
pub struct GetFulfillmentPreviewItem {
  /// The seller SKU of the item.
  pub SellerSKU: String,
  /// A fulfillment order item identifier that you
  /// created with a call to the
  /// GetFulfillmentPreview operation.
  pub SellerFulfillmentOrderItemId: String,
  /// The item quantity.
  pub Quantity: i32,
}

/// Parameters for `GetFulfillmentPreview`
#[allow(non_snake_case)]
#[derive(Debug)]
pub struct GetFulfillmentPreviewParameters {
  pub Address: DestinationAddress,
  pub Items: Vec<GetFulfillmentPreviewItem>,

  // Optional API Parameters
  pub MarketplaceId: Option<String>,
  pub ShippingSpeedCategories: Option<Vec<String>>,
}

impl Into<Vec<(String, String)>> for GetFulfillmentPreviewParameters {
  fn into(self) -> Vec<(String, String)> {
    let mut result = vec![
      ("Address.Name".to_owned(), self.Address.Name),
      ("Address.Line1".to_owned(), self.Address.Line1),
      (
        "Address.StateOrProvinceCode".to_owned(),
        self.Address.StateOrProvinceCode,
      ),
      ("Address.CountryCode".to_owned(), self.Address.CountryCode),
    ];

    if !self.Address.Line2.is_empty() {
      result.push(("Address.Line2".to_owned(), self.Address.Line2))
    }

    if !self.Address.Line3.is_empty() {
      result.push(("Address.Line3".to_owned(), self.Address.Line3))
    }

    if !self.Address.DistrictOrCounty.is_empty() {
      result.push((
        "Address.DistrictOrCounty".to_owned(),
        self.Address.DistrictOrCounty,
      ))
    }

    if !self.Address.City.is_empty() {
      result.push(("Address.City".to_owned(), self.Address.City))
    }

    if !self.Address.PhoneNumber.is_empty() {
      result.push(("Address.PhoneNumber".to_owned(), self.Address.PhoneNumber))
    }

    if !self.Address.PostalCode.is_empty() {
      result.push(("Address.PostalCode".to_owned(), self.Address.PostalCode))
    }

    for (i, item) in self.Items.into_iter().enumerate() {
      result.push((format!("Items.member.{}.SellerSKU", i + 1), item.SellerSKU));
      result.push((
        format!("Items.member.{}.SellerFulfillmentOrderItemId", i + 1),
        item.SellerFulfillmentOrderItemId,
      ));
      result.push((
        format!("Items.member.{}.Quantity", i + 1),
        item.Quantity.to_string(),
      ));
    }

    if let Some(marketplace_id) = self.MarketplaceId {
      result.push(("MarketplaceId".to_owned(), marketplace_id))
    }

    if let Some(cats) = self.ShippingSpeedCategories {
      for (i, cat) in cats.into_iter().enumerate() {
        result.push((format!("ShippingSpeedCategories.member.{}", i + 1), cat))
      }
    }

    result
  }
}

#[derive(Debug, Default)]
#[allow(non_snake_case)]
pub struct GetFulfillmentPreviewResponse {
  pub FulfillmentPreviews: Vec<FulfillmentPreview>,
  pub RequestId: String,
}

impl<S: decode::XmlEventStream> decode::FromXMLStream<S> for GetFulfillmentPreviewResponse {
  fn from_xml(s: &mut S) -> decode::Result<GetFulfillmentPreviewResponse> {
    use self::decode::{characters, element, fold_elements, start_document};
    start_document(s)?;
    element(s, "GetFulfillmentPreviewResponse", |s| {
      fold_elements(
        s,
        GetFulfillmentPreviewResponse::default(),
        |s, response| match s.local_name() {
          "GetFulfillmentPreviewResult" => fold_elements(s, (), |s, _| match s.local_name() {
            "FulfillmentPreviews" => {
              response.FulfillmentPreviews = fold_elements(s, vec![], |s, v| {
                v.push(FulfillmentPreview::from_xml(s)?);
                Ok(())
              })?;
              Ok(())
            }
            _ => Ok(()),
          }),
          "ResponseMetadata" => {
            response.RequestId = element(s, "RequestId", |s| characters(s))?;
            Ok(())
          }
          _ => Ok(()),
        },
      )
    })
  }
}

/// Returns a list of fulfillment order previews based on shipping criteria that you specify.
///
/// [Documentation](https://docs.developer.amazonservices.com/en_US/fba_outbound/FBAOutbound_GetFulfillmentPreview.html)
#[allow(non_snake_case)]
pub fn GetFulfillmentPreview(
  client: &Client,
  params: GetFulfillmentPreviewParameters,
) -> Result<Response<GetFulfillmentPreviewResponse>> {
  client
    .request_xml(Method::Post, PATH, VERSION, "GetFulfillmentPreview", params)
    .map_err(|err| err.into())
}

#[cfg(test)]
mod tests {
  use super::super::client::get_test_client;
  use super::*;
  use dotenv::dotenv;

  #[test]
  #[ignore]
  fn test_get_package_tracking_details() {
    dotenv().ok();
    let c = get_test_client();
    let res = GetPackageTrackingDetails(&c, "187748827").expect("GetPackageTrackingDetails");
    match res {
      Response::Error(e) => panic!("request error: {:?}", e),
      Response::Success(res) => {
        println!("res = {:?}", res);
      }
    }
  }

  #[test]
  fn test_get_fufillment_preview() {
    dotenv().ok();
    let c = get_test_client();
    let res = GetFulfillmentPreview(
      &c,
      GetFulfillmentPreviewParameters {
        Address: DestinationAddress {
          PhoneNumber: "".to_owned(),
          City: "North York".to_owned(),
          CountryCode: "CA".to_owned(),
          PostalCode: "M2N0E9".to_owned(),
          Name: "Foo Bar".to_owned(),
          StateOrProvinceCode: "ON".to_owned(),
          DistrictOrCounty: "".to_owned(),
          Line1: "1000-5162 Yonge Street".to_owned(),
          Line2: "".to_owned(),
          Line3: "".to_owned(),
        },
        Items: vec![
          GetFulfillmentPreviewItem {
            SellerSKU: "edifier-r1280t-fba".to_owned(),
            SellerFulfillmentOrderItemId: "1".to_owned(),
            Quantity: 1,
          },
          GetFulfillmentPreviewItem {
            SellerSKU: "edifier-h210-black".to_owned(),
            SellerFulfillmentOrderItemId: "2".to_owned(),
            Quantity: 139,
          },
        ],
        MarketplaceId: Some("A2EUQ1WTGCTBG2".to_string()),
        ShippingSpeedCategories: Some(vec![
          "Standard".to_owned(),
          "Expedited".to_owned(),
          "Priority".to_owned(),
        ]),
      },
    ).expect("GetFulfillmentPreview");
    match res {
      Response::Error(e) => panic!("request error: {:?}", e),
      Response::Success(res) => {
        println!("res = {:?}", res);
      }
    }
  }
}
