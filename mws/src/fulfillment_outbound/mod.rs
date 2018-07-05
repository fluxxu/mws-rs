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

#[derive(Debug, Default, Serialize)]
pub struct ListAllFulfillmentOrdersResponse {
  pub request_id: String,
  pub fulfillment_orders: Vec<FulfillmentOrder>,
  pub next_token: Option<String>,
}

impl<S: decode::XmlEventStream> decode::FromXmlStream<S> for ListAllFulfillmentOrdersResponse {
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

#[derive(Debug, Default, Serialize)]
#[allow(non_snake_case)]
pub struct GetFulfillmentOrderResponse {
  pub fulfillment_shipments: Vec<FulfillmentShipment>,
  pub return_item_list: Vec<ReturnItemList>,
  pub return_authorization_list: Vec<ReturnAuthorizationList>,
  pub fulfillment_order: FulfillmentOrder,
  pub fulfillment_order_items: Vec<FulfillmentOrderItem>,
  pub request_id: String,
}

impl<S: decode::XmlEventStream> decode::FromXmlStream<S> for GetFulfillmentOrderResponse {
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

#[derive(Debug, Default, Serialize)]
#[allow(non_snake_case)]
pub struct GetPackageTrackingDetailsResponse {
  pub details: PackageTrackingDetails,
  pub request_id: String,
}

impl<S: decode::XmlEventStream> decode::FromXmlStream<S> for GetPackageTrackingDetailsResponse {
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
#[derive(Debug, Default, Serialize)]
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
#[derive(Debug, Serialize)]
pub struct GetFulfillmentPreviewParameters {
  pub Address: DestinationAddress,
  pub Items: Vec<GetFulfillmentPreviewItem>,

  // Optional API Parameters
  pub MarketplaceId: Option<String>,
  pub ShippingSpeedCategories: Option<Vec<ShippingSpeedCategory>>,
}

fn get_address_pairs(prop: &str, addr: &DestinationAddress) -> Vec<(String, String)> {
  let mut result = vec![
    (format!("{}.Name", prop), addr.Name.clone()),
    (format!("{}.Line1", prop), addr.Line1.clone()),
    (
      format!("{}.StateOrProvinceCode", prop),
      addr.StateOrProvinceCode.clone(),
    ),
    (format!("{}.CountryCode", prop), addr.CountryCode.clone()),
  ];

  if !addr.Line2.is_empty() {
    result.push((format!("{}.Line2", prop), addr.Line2.clone()))
  }

  if !addr.Line3.is_empty() {
    result.push((format!("{}.Line3", prop), addr.Line3.clone()))
  }

  if !addr.DistrictOrCounty.is_empty() {
    result.push((
      format!("{}.DistrictOrCounty", prop),
      addr.DistrictOrCounty.clone(),
    ))
  }

  if !addr.City.is_empty() {
    result.push((format!("{}.City", prop), addr.City.clone()))
  }

  if !addr.PhoneNumber.is_empty() {
    result.push((format!("{}.PhoneNumber", prop), addr.PhoneNumber.clone()))
  }

  if !addr.PostalCode.is_empty() {
    result.push((format!("{}.PostalCode", prop), addr.PostalCode.clone()))
  }

  result
}

impl Into<Vec<(String, String)>> for GetFulfillmentPreviewParameters {
  fn into(self) -> Vec<(String, String)> {
    let mut result = vec![];

    result.append(&mut get_address_pairs("Address", &self.Address));

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
        result.push((
          format!("ShippingSpeedCategories.member.{}", i + 1),
          cat.to_string(),
        ))
      }
    }

    result
  }
}

#[derive(Debug, Default, Serialize)]
#[allow(non_snake_case)]
pub struct GetFulfillmentPreviewResponse {
  pub FulfillmentPreviews: Vec<FulfillmentPreview>,
  pub RequestId: String,
}

impl<S: decode::XmlEventStream> decode::FromXmlStream<S> for GetFulfillmentPreviewResponse {
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

/// Item information for creating a fulfillment order.
#[allow(non_snake_case)]
#[derive(Debug, Default, Serialize)]
pub struct CreateFulfillmentOrderItem {
  /// The seller SKU of the item.
  pub SellerSKU: String,
  /// A fulfillment order item identifier that you
  /// created with a call to the
  /// GetFulfillmentPreview operation.
  pub SellerFulfillmentOrderItemId: String,
  /// The item quantity.
  pub Quantity: i32,
  /// A message to the gift recipient, if applicable.
  pub GiftMessage: Option<String>,
  /// Item-specific text that displays in recipient-facing
  /// materials such as the outbound shipment packing slip.
  pub DisplayableComment: Option<String>,
  /// Amazon's fulfillment network SKU of the item.
  pub FulfillmentNetworkSKU: Option<String>,
  /// The monetary value assigned by the seller to this item.
  pub PerUnitDeclaredValue: Option<Currency>,
  /// The amount to be collected from the customer
  /// for this item in a COD (Cash On Delivery) order.
  pub PerUnitPrice: Option<Currency>,
  /// The tax on the amount to be collected from the customer
  /// for this item in a COD (Cash On Delivery) order.
  pub PerUnitTax: Option<Currency>,
}

/// Parameters for `CreateFulfillmentOrder`
#[allow(non_snake_case)]
#[derive(Debug, Serialize)]
pub struct CreateFulfillmentOrderParameters {
  pub SellerFulfillmentOrderId: String,
  pub ShippingSpeedCategory: ShippingSpeedCategory,
  pub DisplayableOrderId: String,
  pub DisplayableOrderDateTime: DateTime<Utc>,
  pub DisplayableOrderComment: String,
  pub DestinationAddress: DestinationAddress,
  pub Items: Vec<CreateFulfillmentOrderItem>,

  // Optional API Parameters
  pub MarketplaceId: Option<String>,
  pub ShipFromCountryCode: Option<String>,
  pub FulfillmentPolicy: Option<FulfillmentPolicy>,
  pub FulfillmentAction: Option<FulfillmentAction>,
  pub NotificationEmailList: Option<Vec<String>>,
}

impl Into<Vec<(String, String)>> for CreateFulfillmentOrderParameters {
  fn into(self) -> Vec<(String, String)> {
    let mut result = vec![
      (
        "SellerFulfillmentOrderId".to_owned(),
        self.SellerFulfillmentOrderId,
      ),
      (
        "ShippingSpeedCategory".to_owned(),
        self.ShippingSpeedCategory.to_string(),
      ),
      ("DisplayableOrderId".to_owned(), self.DisplayableOrderId),
      (
        "DisplayableOrderDateTime".to_owned(),
        self.DisplayableOrderDateTime.to_iso8601(),
      ),
      (
        "DisplayableOrderComment".to_owned(),
        self.DisplayableOrderComment,
      ),
    ];

    result.append(&mut get_address_pairs(
      "DestinationAddress",
      &self.DestinationAddress,
    ));

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

      if let Some(msg) = item.GiftMessage {
        result.push((format!("Items.member.{}.GiftMessage", i + 1), msg));
      }

      if let Some(msg) = item.DisplayableComment {
        result.push((format!("Items.member.{}.DisplayableComment", i + 1), msg));
      }

      if let Some(msg) = item.FulfillmentNetworkSKU {
        result.push((format!("Items.member.{}.FulfillmentNetworkSKU", i + 1), msg));
      }

      if let Some(c) = item.PerUnitDeclaredValue {
        result.push((
          format!("Items.member.{}.PerUnitDeclaredValue.CurrencyCode", i + 1),
          c.CurrencyCode,
        ));
        result.push((
          format!("Items.member.{}.PerUnitDeclaredValue.Value", i + 1),
          c.Value,
        ));
      }

      if let Some(c) = item.PerUnitPrice {
        result.push((
          format!("Items.member.{}.PerUnitPrice.CurrencyCode", i + 1),
          c.CurrencyCode,
        ));
        result.push((
          format!("Items.member.{}.PerUnitPrice.Value", i + 1),
          c.Value,
        ));
      }

      if let Some(c) = item.PerUnitTax {
        result.push((
          format!("Items.member.{}.PerUnitTax.CurrencyCode", i + 1),
          c.CurrencyCode,
        ));
        result.push((format!("Items.member.{}.PerUnitTax.Value", i + 1), c.Value));
      }
    }

    if let Some(marketplace_id) = self.MarketplaceId {
      result.push(("MarketplaceId".to_owned(), marketplace_id))
    }

    if let Some(code) = self.ShipFromCountryCode {
      result.push(("ShipFromCountryCode".to_owned(), code))
    }

    if let Some(v) = self.FulfillmentPolicy {
      result.push(("FulfillmentPolicy".to_owned(), v.to_string()))
    }

    if let Some(v) = self.FulfillmentAction {
      result.push(("FulfillmentAction".to_owned(), v.to_string()))
    }

    if let Some(emails) = self.NotificationEmailList {
      for (i, email) in emails.into_iter().enumerate() {
        result.push((format!("NotificationEmailList.member.{}", i + 1), email))
      }
    }

    result
  }
}

#[derive(Debug, Default, Serialize)]
#[allow(non_snake_case)]
pub struct CreateFulfillmentOrderResponse {
  pub RequestId: String,
}

impl<S: decode::XmlEventStream> decode::FromXmlStream<S> for CreateFulfillmentOrderResponse {
  fn from_xml(s: &mut S) -> decode::Result<CreateFulfillmentOrderResponse> {
    use self::decode::{characters, element, fold_elements, start_document};
    start_document(s)?;
    element(s, "CreateFulfillmentOrderResponse", |s| {
      fold_elements(
        s,
        CreateFulfillmentOrderResponse::default(),
        |s, response| match s.local_name() {
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

/// Requests that Amazon ship items from the seller's inventory in Amazon's fulfillment network to a destination address.
///
/// [Documentation](https://docs.developer.amazonservices.com/en_US/fba_outbound/FBAOutbound_CreateFulfillmentOrder.html)
#[allow(non_snake_case)]
pub fn CreateFulfillmentOrder(
  client: &Client,
  params: CreateFulfillmentOrderParameters,
) -> Result<Response<CreateFulfillmentOrderResponse>> {
  client
    .request_xml(
      Method::Post,
      PATH,
      VERSION,
      "CreateFulfillmentOrder",
      params,
    )
    .map_err(|err| err.into())
}

#[derive(Debug, Default, Serialize)]
#[allow(non_snake_case)]
pub struct CancelFulfillmentOrderResponse {
  pub RequestId: String,
}

impl<S: decode::XmlEventStream> decode::FromXmlStream<S> for CancelFulfillmentOrderResponse {
  fn from_xml(s: &mut S) -> decode::Result<CancelFulfillmentOrderResponse> {
    use self::decode::{characters, element, fold_elements, start_document};
    start_document(s)?;
    element(s, "CancelFulfillmentOrderResponse", |s| {
      fold_elements(
        s,
        CancelFulfillmentOrderResponse::default(),
        |s, response| match s.local_name() {
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

/// Requests that Amazon stop attempting to fulfill an existing fulfillment order.
///
/// [Documentation](https://docs.developer.amazonservices.com/en_US/fba_outbound/FBAOutbound_CancelFulfillmentOrder.html)
#[allow(non_snake_case)]
pub fn CancelFulfillmentOrder(
  client: &Client,
  seller_fulfillment_order_id: &str,
) -> Result<Response<CancelFulfillmentOrderResponse>> {
  client
    .request_xml(
      Method::Post,
      PATH,
      VERSION,
      "CancelFulfillmentOrder",
      vec![(
        "SellerFulfillmentOrderId".to_owned(),
        seller_fulfillment_order_id.to_owned(),
      )],
    )
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
  #[ignore]
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
          Line1: "1000-7777 Yonge Street".to_owned(),
          Line2: "".to_owned(),
          Line3: "".to_owned(),
        },
        Items: vec![
          GetFulfillmentPreviewItem {
            SellerSKU: "e1".to_owned(),
            SellerFulfillmentOrderItemId: "1".to_owned(),
            Quantity: 1,
          },
          GetFulfillmentPreviewItem {
            SellerSKU: "e2".to_owned(),
            SellerFulfillmentOrderItemId: "2".to_owned(),
            Quantity: 139,
          },
        ],
        MarketplaceId: Some("A2EUQ1WTGCTBG2".to_string()),
        ShippingSpeedCategories: Some(vec![
          ShippingSpeedCategory::Standard,
          ShippingSpeedCategory::Expedited,
          ShippingSpeedCategory::Priority,
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

  #[test]
  #[ignore]
  fn test_create_fulfillment_order() {
    use chrono::TimeZone;

    dotenv().ok();
    let c = get_test_client();
    let res = CreateFulfillmentOrder(
      &c,
      CreateFulfillmentOrderParameters {
        SellerFulfillmentOrderId: "S2_TEST_20180517_3".to_owned(),
        ShippingSpeedCategory: ShippingSpeedCategory::Expedited,
        DisplayableOrderId: "TEST".to_owned(),
        DisplayableOrderDateTime: Utc.ymd(2018, 5, 5).and_hms(0, 0, 0),
        DisplayableOrderComment: "DisplayableOrderComment".to_owned(),
        DestinationAddress: DestinationAddress {
          PhoneNumber: "".to_owned(),
          City: "North York".to_owned(),
          CountryCode: "CA".to_owned(),
          PostalCode: "M2N0E9".to_owned(),
          Name: "F A".to_owned(),
          StateOrProvinceCode: "ON".to_owned(),
          DistrictOrCounty: "".to_owned(),
          Line1: "88-888 Yonge Street".to_owned(),
          Line2: "".to_owned(),
          Line3: "".to_owned(),
        },
        Items: vec![CreateFulfillmentOrderItem {
          SellerSKU: "e-fba".to_owned(),
          SellerFulfillmentOrderItemId: "1".to_owned(),
          Quantity: 1,
          GiftMessage: Some("GiftMessage".to_owned()),
          DisplayableComment: Some("DisplayableComment".to_owned()),
          FulfillmentNetworkSKU: None,
          PerUnitDeclaredValue: None,
          PerUnitPrice: None,
          PerUnitTax: None,
        }],
        MarketplaceId: Some("A2EUQ1WTGCTBG2".to_string()),
        ShipFromCountryCode: None,
        FulfillmentPolicy: Some(FulfillmentPolicy::FillOrKill),
        FulfillmentAction: Some(FulfillmentAction::Hold),
        NotificationEmailList: Some(vec!["a@gmail.com".to_owned(), "b@ventmere.com".to_owned()]),
      },
    ).expect("CreateFulfillmentOrder");
    match res {
      Response::Error(e) => panic!("request error: {:?}", e),
      Response::Success(res) => {
        println!("res = {:?}", res);
      }
    }
  }

  #[test]
  #[ignore]
  fn test_cancel_fulfillment_order() {
    dotenv().ok();
    let c = get_test_client();
    let res = CancelFulfillmentOrder(&c, "S2_TEST_20180517_3").expect("CancelFulfillmentOrder");
    match res {
      Response::Error(e) => panic!("request error: {:?}", e),
      Response::Success(res) => {
        println!("res = {:?}", res);
      }
    }
  }
}
