//! Merchant Fulfillment - Version 2015-06-01
//!
//! [Documentation](http://docs.developer.amazonservices.com/en_US/merch_fulfill/MerchFulfill_Overview.html)

pub use self::types::*;
use client::{Client, Method};
use result::MwsResult;

mod types;

static PATH: &'static str = "/MerchantFulfillment/2015-06-01";
static VERSION: &'static str = "2015-06-01";

#[derive(FromXmlStream, Default, Debug)]
#[allow(non_snake_case)]
pub struct GetEligibleShippingServicesResponse {
  pub ShippingServiceList: Vec<ShippingService>,
  pub TermsAndConditionsNotAcceptedCarrierList: Vec<Carrier>,
  pub TemporarilyUnavailableCarrierList: Vec<Carrier>,
}

response_envelope_type!(
  GetEligibleShippingServicesEnvelope<GetEligibleShippingServicesResponse>,
  "GetEligibleShippingServicesResponse",
  "GetEligibleShippingServicesResult"
);

#[allow(non_snake_case)]
#[derive(Debug, Default, Serialize, SerializeMwsParams)]
pub struct GetEligibleShippingServicesParameters {
  pub ShipmentRequestDetails: ShipmentRequestDetails,
}

/// Returns a list of shipping service offers.
///
/// [Documentation](http://docs.developer.amazonservices.com/en_US/merch_fulfill/MerchFulfill_GetEligibleShippingServices.html)
#[allow(non_snake_case)]
pub fn GetEligibleShippingServices(
  client: &Client,
  params: GetEligibleShippingServicesParameters,
) -> MwsResult<GetEligibleShippingServicesResponse> {
  client
    .request_xml(
      Method::Post,
      PATH,
      VERSION,
      "GetEligibleShippingServices",
      params,
    )
    .map(|e: GetEligibleShippingServicesEnvelope| e.into_inner())
    .map_err(|err| err.into())
}

#[derive(FromXmlStream, Default, Debug)]
#[allow(non_snake_case)]
pub struct CreateShipmentResponse {
  pub Shipment: Shipment,
}

response_envelope_type!(
  CreateShipmentEnvelope<CreateShipmentResponse>,
  "CreateShipmentResponse",
  "CreateShipmentResult"
);

#[allow(non_snake_case)]
#[derive(Debug, Default, Serialize, SerializeMwsParams)]
pub struct CreateShipmentParameters {
  pub ShipmentRequestDetails: ShipmentRequestDetails,
  pub ShippingServiceId: String,
  pub ShippingServiceOfferId: Option<String>,
  pub HazmatType: Option<HazmatType>,
}

/// The CreateShipment operation purchases shipping and
/// returns PDF, PNG, or ZPL document data for a shipping
/// label, depending on the carrier.
///
/// [Documentation](http://docs.developer.amazonservices.com/en_CA/merch_fulfill/MerchFulfill_CreateShipment.html)
#[allow(non_snake_case)]
pub fn CreateShipment(
  client: &Client,
  params: CreateShipmentParameters,
) -> MwsResult<CreateShipmentResponse> {
  client
    .request_xml(Method::Post, PATH, VERSION, "CreateShipment", params)
    .map(|e: CreateShipmentEnvelope| e.into_inner())
    .map_err(|err| err.into())
}

#[derive(FromXmlStream, Default, Debug)]
#[allow(non_snake_case)]
pub struct GetShipmentResponse {
  pub Shipment: Shipment,
}

response_envelope_type!(
  GetShipmentEnvelope<GetShipmentResponse>,
  "GetShipmentResponse",
  "GetShipmentResult"
);

#[allow(non_snake_case)]
pub fn GetShipment(client: &Client, id: &str) -> MwsResult<GetShipmentResponse> {
  client
    .request_xml(
      Method::Post,
      PATH,
      VERSION,
      "GetShipment",
      vec![("ShipmentId".to_string(), id.to_string())],
    )
    .map(|e: GetShipmentEnvelope| e.into_inner())
    .map_err(|err| err.into())
}

#[derive(FromXmlStream, Default, Debug)]
#[allow(non_snake_case)]
pub struct CancelShipmentResponse {
  pub Shipment: Shipment,
}

response_envelope_type!(
  CancelShipmentEnvelope<CancelShipmentResponse>,
  "CancelShipmentResponse",
  "CancelShipmentResult"
);

#[allow(non_snake_case)]
pub fn CancelShipment(client: &Client, id: &str) -> MwsResult<CancelShipmentResponse> {
  client
    .request_xml(
      Method::Post,
      PATH,
      VERSION,
      "CancelShipment",
      vec![("ShipmentId".to_string(), id.to_string())],
    )
    .map(|e: CancelShipmentEnvelope| e.into_inner())
    .map_err(|err| err.into())
}

#[cfg(test)]
mod tests {
  use super::super::client::get_test_client;
  use super::*;
  use dotenv::dotenv;

  fn get_test_details() -> ShipmentRequestDetails {
    ShipmentRequestDetails {
      AmazonOrderId: "114-3620592-9701026".to_owned(),
      SellerOrderId: Some("418465".to_owned()),
      ItemList: vec![Item {
        OrderItemId: "70203066456690".to_owned(),
        Quantity: 1,
      }],
      ShipFromAddress: Address {
        Name: "RPD Ltd.".to_owned(),
        AddressLine1: "428 Hemphill Street".to_owned(),
        AddressLine2: "".to_owned(),
        AddressLine3: "".to_owned(),
        DistrictOrCounty: "".to_owned(),
        Email: "orders@ventmere.com".to_owned(),
        City: "Fort Worth".to_owned(),
        StateOrProvinceCode: "Texas".to_owned(),
        PostalCode: "76104".to_owned(),
        CountryCode: "US".to_owned(),
        Phone: "8176652160".to_owned(),
      },
      PackageDimensions: PackageDimensions {
        Length: "37.8".to_owned(),
        Width: "23.8".to_owned(),
        Height: "34.4".to_owned(),
        Unit: DimensionsUnit::centimeters,
        PredefinedPackageDimensions: None,
      },
      Weight: Weight {
        Value: "8000".to_owned(),
        Unit: WeightUnit::g,
      },
      Insurance: None,
      MustArriveByDate: None,
      ShipDate: None,
      ShippingServiceOptions: ShippingServiceOptions {
        DeliveryExperience: DeliveryExperience::DeliveryConfirmationWithSignature,
        DeclaredValue: None,
        CarrierWillPickUp: false,
        LabelFormat: None,
      },
      LabelCustomization: Default::default(),
    }
  }

  #[test]
  #[ignore]
  fn test_get_eligible_shipping_services() {
    dotenv().ok();
    let c = get_test_client();
    let details = get_test_details();

    let res = GetEligibleShippingServices(
      &c,
      GetEligibleShippingServicesParameters {
        ShipmentRequestDetails: details,
      },
    )
    .expect("GetEligibleShippingServices");
    println!("res = {:#?}", res);
  }

  #[test]
  #[ignore]
  fn test_create_shipment() {
    dotenv().ok();
    let c = get_test_client();
    let params = CreateShipmentParameters {
      ShipmentRequestDetails: get_test_details(),
      ShippingServiceId: "USPS_PTP_EXP".to_owned(),
      ShippingServiceOfferId: Some("o0OaoPEue25v0FBaYQ4JvRD9LalsRiwurLkVNk98ZPM73yL/Li9qEHtleIqHfOMDvCc7GjsyOgGtNpOlSVAsomTnIG/TVBerhIEScCCmgKWlY4+TPJIZatfyq3y2BBNOTH9JEXpRj9TLkJLsmf6A23X8FWpeUtWYcS8e2A0019o=".to_owned()),
      HazmatType: Some(HazmatType::None),
    };
    let res = CreateShipment(&c, params).expect("CreateShipment");
    println!("res = {:#?}", res);
  }

  #[test]
  #[ignore]
  fn test_get_shipment() {
    dotenv().ok();
    let c = get_test_client();

    let res = GetShipment(&c, "a4062ca8-faa0-49d4-bb0b-b32433ebdb3a").expect("GetShipment");
    println!("res = {:#?}", res);
  }

  #[test]
  #[ignore]
  fn test_cancel_shipment() {
    dotenv().ok();
    let c = get_test_client();

    let res = CancelShipment(&c, "a4062ca8-faa0-49d4-bb0b-b32433ebdb3a").expect("CancelShipment");
    println!("res = {:#?}", res);
  }
}
