//! Amazon MWS Fulfillment Inbound Shipment API - Version 2010-10-01
//!
//! [Documentation](http://docs.developer.amazonservices.com/en_CA/fba_inbound/FBAInbound_Overview.html)

use chrono::{DateTime, Utc};
use client::{Client, Method};
use result::MwsResult;

mod types;
pub use self::types::*;

static PATH: &'static str = "/FulfillmentInboundShipment/2010-10-01";
static VERSION: &'static str = "2010-10-01";

/// Parameters for `ListInboundShipments`
#[allow(non_snake_case)]
#[derive(Debug, Default, Serialize, SerializeMwsParams)]
pub struct ListInboundShipmentsParameters {
  pub ShipmentStatusList: Vec<ShipmentStatus>,
  pub ShipmentIdList: Vec<String>,
  pub LastUpdatedAfter: Option<DateTime<Utc>>,
  pub LastUpdatedBefore: Option<DateTime<Utc>>,
}

#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq, Serialize, FromXmlStream)]
pub struct ListInboundShipmentsResponse {
  pub ShipmentData: Vec<InboundShipmentInfo>,
  pub NextToken: Option<String>,
}

response_envelope_type!(
  ListInboundShipmentsEnvelope<ListInboundShipmentsResponse>,
  "ListInboundShipmentsResponse",
  "ListInboundShipmentsResult"
);

response_envelope_type!(
  ListInboundShipmentsByNextTokenEnvelope<ListInboundShipmentsResponse>,
  "ListInboundShipmentsByNextTokenResponse",
  "ListInboundShipmentsByNextTokenResult"
);

/// Returns a list of inbound shipments based on criteria that you specify.
///
/// [Documentation](http://docs.developer.amazonservices.com/en_CA/fba_inbound/FBAInbound_ListInboundShipments.html)
#[allow(non_snake_case)]
pub fn ListInboundShipments(
  client: &Client,
  parameters: ListInboundShipmentsParameters,
) -> MwsResult<ListInboundShipmentsResponse> {
  client
    .request_xml(
      Method::Post,
      PATH,
      VERSION,
      "ListInboundShipments",
      parameters,
    )
    .map(|e: ListInboundShipmentsEnvelope| e.into_inner())
    .map_err(|err| err.into())
}

/// Returns the next page of inbound shipments using the NextToken parameter.
///
/// [Documentation](http://docs.developer.amazonservices.com/en_CA/fba_inbound/FBAInbound_ListInboundShipmentsByNextToken.html)
#[allow(non_snake_case)]
pub fn ListInboundShipmentsByNextToken(
  client: &Client,
  next_token: String,
) -> MwsResult<ListInboundShipmentsResponse> {
  let params = vec![("NextToken".to_string(), next_token)];
  client
    .request_xml(
      Method::Post,
      PATH,
      VERSION,
      "ListInboundShipmentsByNextToken",
      params,
    )
    .map(|e: ListInboundShipmentsByNextTokenEnvelope| e.into_inner())
    .map_err(|err| err.into())
}

/// Parameters for `ListInboundShipments`
#[allow(non_snake_case)]
#[derive(Debug, Default, Serialize, SerializeMwsParams)]
pub struct ListInboundShipmentItemsParameters {
  pub ShipmentId: String,
  pub LastUpdatedAfter: Option<DateTime<Utc>>,
  pub LastUpdatedBefore: Option<DateTime<Utc>>,
}

#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq, Serialize, FromXmlStream)]
pub struct ListInboundShipmentItemsResponse {
  pub ItemData: Vec<InboundShipmentItem>,
  pub NextToken: Option<String>,
}

response_envelope_type!(
  ListInboundShipmentItemsEnvelope<ListInboundShipmentItemsResponse>,
  "ListInboundShipmentItemsResponse",
  "ListInboundShipmentItemsResult"
);

response_envelope_type!(
  ListInboundShipmentItemsByNextTokenEnvelope<ListInboundShipmentItemsResponse>,
  "ListInboundShipmentItemsByNextTokenResponse",
  "ListInboundShipmentItemsByNextTokenResult"
);

/// Returns a list of items in a specified inbound shipment, or a list of items that were updated within a specified time frame.
///
/// [Documentation](http://docs.developer.amazonservices.com/en_CA/fba_inbound/FBAInbound_ListInboundShipmentItems.html)
#[allow(non_snake_case)]
pub fn ListInboundShipmentItems(
  client: &Client,
  parameters: ListInboundShipmentItemsParameters,
) -> MwsResult<ListInboundShipmentItemsResponse> {
  client
    .request_xml(
      Method::Post,
      PATH,
      VERSION,
      "ListInboundShipmentItems",
      parameters,
    )
    .map(|e: ListInboundShipmentItemsEnvelope| e.into_inner())
    .map_err(|err| err.into())
}

/// Returns the next page of inbound shipment items using the NextToken parameter.
///
/// [Documentation](http://docs.developer.amazonservices.com/en_CA/fba_inbound/FBAInbound_ListInboundShipmentItemsByNextToken.html)
#[allow(non_snake_case)]
pub fn ListInboundShipmentItemsByNextToken(
  client: &Client,
  next_token: String,
) -> MwsResult<ListInboundShipmentItemsResponse> {
  let params = vec![("NextToken".to_string(), next_token)];
  client
    .request_xml(
      Method::Post,
      PATH,
      VERSION,
      "ListInboundShipmentItemsByNextToken",
      params,
    )
    .map(|e: ListInboundShipmentItemsByNextTokenEnvelope| e.into_inner())
    .map_err(|err| err.into())
}

#[cfg(test)]
mod tests {
  use super::*;
  use types::SerializeMwsParams;

  #[test]
  fn test_encode_list_inbound_shipments_params() {
    let params = ListInboundShipmentsParameters {
      ShipmentStatusList: vec![ShipmentStatus::WORKING, ShipmentStatus::DELETED],
      ShipmentIdList: vec!["001".to_owned(), "002".to_owned()],
      LastUpdatedAfter: None,
      LastUpdatedBefore: None,
    };
    assert_eq!(
      params.into_mws_params(),
      vec![
        (
          "ShipmentStatusList.member.1".to_owned(),
          "WORKING".to_owned()
        ),
        (
          "ShipmentStatusList.member.2".to_owned(),
          "DELETED".to_owned()
        ),
        ("ShipmentIdList.member.1".to_owned(), "001".to_owned()),
        ("ShipmentIdList.member.2".to_owned(), "002".to_owned()),
      ]
    )
  }

  #[test]
  fn test_decode_list_inbound_shipments_response() {
    test_decode_envelope!(
      ListInboundShipmentsEnvelope,
      r#"
    <ListInboundShipmentsResponse xmlns="http://mws.amazonaws.com/FulfillmentInboundShipment/2010-10-01/">
      <ListInboundShipmentsResult>
        <ShipmentData>
          <member>
            <DestinationFulfillmentCenterId>MDW2</DestinationFulfillmentCenterId>
            <LabelPrepType>NO_LABEL</LabelPrepType>
            <ShipFromAddress>
              <City>Fort Worth</City>
              <CountryCode>US</CountryCode>
              <PostalCode>76104</PostalCode>
              <Name>RPD LTD</Name>
              <AddressLine1>428 Hemphill St</AddressLine1>
              <StateOrProvinceCode>TX</StateOrProvinceCode>
            </ShipFromAddress>
            <ShipmentId>FBA4FWJMVV</ShipmentId>
            <AreCasesRequired>true</AreCasesRequired>
            <ShipmentName>FBA (2/19/17 9:39 PM) - 1</ShipmentName>
            <BoxContentsSource>INTERACTIVE</BoxContentsSource>
            <ShipmentStatus>CLOSED</ShipmentStatus>
          </member>
        </ShipmentData>
      </ListInboundShipmentsResult>
      <ResponseMetadata>
        <RequestId>04c87e79-f747-4da9-984f-5bc1f0b875e6</RequestId>
      </ResponseMetadata>
    </ListInboundShipmentsResponse>
    "#,
      ListInboundShipmentsResponse {
        NextToken: None,
        ShipmentData: vec![InboundShipmentInfo {
          DestinationFulfillmentCenterId: "MDW2".to_owned(),
          LabelPrepType: Some(LabelPrepType::NO_LABEL),
          ShipFromAddress: Address {
            City: "Fort Worth".to_owned(),
            CountryCode: "US".to_owned(),
            PostalCode: "76104".to_owned(),
            Name: "RPD LTD".to_owned(),
            AddressLine1: "428 Hemphill St".to_owned(),
            AddressLine2: "".to_owned(),
            StateOrProvinceCode: "TX".to_owned(),
            DistrictOrCounty: "".to_owned(),
          },
          ShipmentId: "FBA4FWJMVV".to_owned(),
          AreCasesRequired: true,
          ShipmentName: "FBA (2/19/17 9:39 PM) - 1".to_owned(),
          BoxContentsSource: Some(BoxContentsSource::INTERACTIVE),
          ShipmentStatus: ShipmentStatus::CLOSED,
          ConfirmedNeedByDate: None,
          EstimatedBoxContentsFee: None,
        }],
      }
    );
  }

  #[test]
  fn test_decode_list_inbound_shipment_items_response() {
    test_decode_envelope!(
      ListInboundShipmentItemsEnvelope,
      r#"
    <ListInboundShipmentItemsResponse xmlns="http://mws.amazonaws.com/FulfillmentInboundShipment/2010-10-01/">
      <ListInboundShipmentItemsResult>
        <ItemData>
          <member>
            <QuantityShipped>60</QuantityShipped>
            <ShipmentId>FBA3T68MQL</ShipmentId>
            <PrepDetailsList>
              <PrepDetails/>
            </PrepDetailsList>
            <FulfillmentNetworkSKU>B016P9HJIA</FulfillmentNetworkSKU>
            <SellerSKU>edifier-r1280t-fba</SellerSKU>
            <QuantityReceived>50</QuantityReceived>
            <QuantityInCase>2</QuantityInCase>
          </member>
        </ItemData>
      </ListInboundShipmentItemsResult>
      <ResponseMetadata>
        <RequestId>70a60f01-d0df-4a29-b093-e1cb53bd8fc2</RequestId>
      </ResponseMetadata>
    </ListInboundShipmentItemsResponse>
    "#,
      ListInboundShipmentItemsResponse {
        NextToken: None,
        ItemData: vec![InboundShipmentItem {
          ShipmentId: "FBA3T68MQL".to_owned(),
          SellerSKU: "edifier-r1280t-fba".to_owned(),
          QuantityShipped: 60,
          QuantityInCase: Some(2),
          QuantityReceived: Some(50),
          FulfillmentNetworkSKU: "B016P9HJIA".to_owned(),
        }],
      }
    );
  }
}
