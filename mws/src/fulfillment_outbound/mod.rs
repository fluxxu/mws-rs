//! Fulfillment Outbound Shipment API - Version 2010-10-01
//!
//! [Documentation](https://docs.developer.amazonservices.com/en_US/fba_outbound/FBAOutbound_Overview.html)

use chrono::{DateTime, Utc};
use client::{Client, Method};
mod types;
pub use self::types::*;
use super::types::ToIso8601;
use result::MwsResult;

static PATH: &'static str = "/FulfillmentOutboundShipment/2010-10-01";
static VERSION: &'static str = "2010-10-01";

#[allow(non_snake_case)]
#[derive(Debug, Default, Serialize, FromXmlStream)]
pub struct ListAllFulfillmentOrdersResponse {
  pub FulfillmentOrders: Vec<FulfillmentOrder>,
  pub NextToken: Option<String>,
}

response_envelope_type!(
  ListAllFulfillmentOrdersEnvelope<ListAllFulfillmentOrdersResponse>,
  "ListAllFulfillmentOrdersResponse",
  "ListAllFulfillmentOrdersResult"
);

response_envelope_type!(
  ListAllFulfillmentOrdersByNextTokenEnvelope<ListAllFulfillmentOrdersResponse>,
  "ListAllFulfillmentOrdersByNextTokenResponse",
  "ListAllFulfillmentOrdersByNextTokenResult"
);

/// Returns a list of fulfillment orders fulfilled after (or at) a specified date.
///
/// [Documentation](http://docs.developer.amazonservices.com/en_US/orders-2013-09-01/Orders_ListOrders.html)
#[allow(non_snake_case)]
pub fn ListAllFulfillmentOrders(
  client: &Client,
  query_start_date_time: DateTime<Utc>,
) -> MwsResult<ListAllFulfillmentOrdersResponse> {
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
    ).map(|e: ListAllFulfillmentOrdersEnvelope| e.into_inner())
    .map_err(|err| err.into())
}

/// Returns the next page of fulfillment orders using the NextToken parameter.
///
/// [Documentation](https://docs.developer.amazonservices.com/en_US/fba_outbound/FBAOutbound_ListAllFulfillmentOrdersByNextToken.html)
#[allow(non_snake_case)]
pub fn ListAllFulfillmentOrdersByNextToken(
  client: &Client,
  next_token: String,
) -> MwsResult<ListAllFulfillmentOrdersResponse> {
  let params = vec![("NextToken".to_string(), next_token)];
  client
    .request_xml(
      Method::Post,
      PATH,
      VERSION,
      "ListAllFulfillmentOrdersByNextToken",
      params,
    ).map(|e: ListAllFulfillmentOrdersByNextTokenEnvelope| e.into_inner())
    .map_err(|err| err.into())
}

response_envelope_type!(
  GetFulfillmentOrderEnvelope<GetFulfillmentOrderResponse>,
  "GetFulfillmentOrderResponse",
  "GetFulfillmentOrderResult"
);

#[derive(Debug, Default, Serialize, FromXmlStream)]
#[allow(non_snake_case)]
pub struct GetFulfillmentOrderResponse {
  pub FulfillmentShipment: Vec<FulfillmentShipment>,
  // pub ReturnItemList: Vec<ReturnItemList>,
  // pub ReturnAuthorizationList: Vec<ReturnAuthorizationList>,
  pub FulfillmentOrder: FulfillmentOrder,
  pub FulfillmentOrderItem: Vec<FulfillmentOrderItem>,
}

/// Returns a fulfillment order based on a specified SellerFulfillmentOrderId.
///
/// [Documentation](http://docs.developer.amazonservices.com/en_CA/fba_outbound/FBAOutbound_GetFulfillmentOrder.html)
#[allow(non_snake_case)]
pub fn GetFulfillmentOrder(
  client: &Client,
  seller_fulfillment_order_id: String,
) -> MwsResult<GetFulfillmentOrderResponse> {
  let params = vec![(
    "SellerFulfillmentOrderId".to_string(),
    seller_fulfillment_order_id,
  )];
  client
    .request_xml(Method::Post, PATH, VERSION, "GetFulfillmentOrder", params)
    .map(|e: GetFulfillmentOrderEnvelope| e.into_inner())
    .map_err(|err| err.into())
}

pub type GetPackageTrackingDetailsResponse = PackageTrackingDetails;

response_envelope_type!(
  GetPackageTrackingDetailsEnvelope<GetPackageTrackingDetailsResponse>,
  "GetPackageTrackingDetailsResponse",
  "GetPackageTrackingDetailsResult"
);

/// Returns delivery tracking information for a package in an outbound shipment for a Multi-Channel Fulfillment order.
///
/// [Documentation](http://docs.developer.amazonservices.com/en_US/fba_outbound/FBAOutbound_GetPackageTrackingDetails.html)
#[allow(non_snake_case)]
pub fn GetPackageTrackingDetails(
  client: &Client,
  package_number: &str,
) -> MwsResult<GetPackageTrackingDetailsResponse> {
  let params = vec![("PackageNumber".to_string(), package_number.to_owned())];
  client
    .request_xml(
      Method::Post,
      PATH,
      VERSION,
      "GetPackageTrackingDetails",
      params,
    ).map(|e: GetPackageTrackingDetailsEnvelope| e.into_inner())
    .map_err(|err| err.into())
}

/// Item information for a fulfillment order preview.
#[allow(non_snake_case)]
#[derive(Debug, Default, Serialize, SerializeMwsParams)]
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
#[derive(Debug, Serialize, SerializeMwsParams)]
pub struct GetFulfillmentPreviewParameters {
  pub Address: DestinationAddress,
  pub Items: Vec<GetFulfillmentPreviewItem>,

  // Optional API Parameters
  pub MarketplaceId: Option<String>,
  pub ShippingSpeedCategories: Option<Vec<ShippingSpeedCategory>>,
}

#[derive(Debug, Default, Serialize, FromXmlStream)]
#[allow(non_snake_case)]
pub struct GetFulfillmentPreviewResponse {
  pub FulfillmentPreviews: Vec<FulfillmentPreview>,
}

response_envelope_type!(
  GetFulfillmentPreviewEnvelope<GetFulfillmentPreviewResponse>,
  "GetFulfillmentPreviewResponse",
  "GetFulfillmentPreviewResult"
);

/// Returns a list of fulfillment order previews based on shipping criteria that you specify.
///
/// [Documentation](https://docs.developer.amazonservices.com/en_US/fba_outbound/FBAOutbound_GetFulfillmentPreview.html)
#[allow(non_snake_case)]
pub fn GetFulfillmentPreview(
  client: &Client,
  params: GetFulfillmentPreviewParameters,
) -> MwsResult<GetFulfillmentPreviewResponse> {
  client
    .request_xml(Method::Post, PATH, VERSION, "GetFulfillmentPreview", params)
    .map(|e: GetFulfillmentPreviewEnvelope| e.into_inner())
    .map_err(|err| err.into())
}

/// Item information for creating a fulfillment order.
#[allow(non_snake_case)]
#[derive(Debug, Default, Serialize, SerializeMwsParams)]
pub struct CreateFulfillmentOrderItem {
  /// The seller SKU of the item.
  pub SellerSKU: String,
  /// Amazon's fulfillment network SKU of the item.
  pub FulfillmentNetworkSKU: Option<String>,
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
#[derive(Debug, Serialize, SerializeMwsParams)]
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

response_envelope_type!(
  CreateFulfillmentOrderEnvelope<()>,
  "CreateFulfillmentOrderResponse",
  "CreateFulfillmentOrderResult"
);

/// Requests that Amazon ship items from the seller's inventory in Amazon's fulfillment network to a destination address.
///
/// [Documentation](https://docs.developer.amazonservices.com/en_US/fba_outbound/FBAOutbound_CreateFulfillmentOrder.html)
#[allow(non_snake_case)]
pub fn CreateFulfillmentOrder(
  client: &Client,
  params: CreateFulfillmentOrderParameters,
) -> MwsResult<()> {
  client
    .request_xml(
      Method::Post,
      PATH,
      VERSION,
      "CreateFulfillmentOrder",
      params,
    ).map(|e: CreateFulfillmentOrderEnvelope| e.into_inner())
    .map_err(|err| err.into())
}

response_envelope_type!(
  CancelFulfillmentOrderEnvelope<()>,
  "CancelFulfillmentOrderResponse",
  "CancelFulfillmentOrderResult"
);

/// Requests that Amazon stop attempting to fulfill an existing fulfillment order.
///
/// [Documentation](https://docs.developer.amazonservices.com/en_US/fba_outbound/FBAOutbound_CancelFulfillmentOrder.html)
#[allow(non_snake_case)]
pub fn CancelFulfillmentOrder(client: &Client, seller_fulfillment_order_id: &str) -> MwsResult<()> {
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
    ).map(|e: CancelFulfillmentOrderEnvelope| e.into_inner())
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
    println!("res = {:#?}", res);
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
    println!("res = {:#?}", res);
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
    println!("res = {:#?}", res);
  }

  #[test]
  #[ignore]
  fn test_cancel_fulfillment_order() {
    dotenv().ok();
    let c = get_test_client();
    let res = CancelFulfillmentOrder(&c, "S2_TEST_20180517_3").expect("CancelFulfillmentOrder");
    println!("res = {:#?}", res);
  }
}
