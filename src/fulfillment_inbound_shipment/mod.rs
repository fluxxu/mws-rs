//! Amazon MWS Fulfillment Inbound Shipment API - Version 2010-10-01
//!
//! [Documentation](http://docs.developer.amazonservices.com/en_CA/fba_inbound/FBAInbound_Overview.html)

use chrono::{DateTime, UTC};
use client::{Client, Method, Response};
use xmlhelper::decode;
use super::types::ToIso8601;

mod types;
pub use self::types::*;

error_chain! {
  links {
    Client(super::client::Error, super::client::ErrorKind);
    Decode(decode::Error, decode::ErrorKind);
  }
}

static PATH: &'static str = "/FulfillmentInboundShipment/2010-10-01";
static VERSION: &'static str = "2010-10-01";

/// Parameters for `ListInboundShipments`
#[derive(Debug, Default)]
pub struct ListInboundShipmentsParameters {
  pub shipment_status_list: Vec<ShipmentStatus>,
  pub shipment_id_list: Vec<String>,
  pub last_updated_after: Option<DateTime<UTC>>,
  pub last_updated_before: Option<DateTime<UTC>>,
}

impl Into<Vec<(String, String)>> for ListInboundShipmentsParameters {
  fn into(self) -> Vec<(String, String)> {
    let mut result = vec![];
    for (i, s) in self.shipment_status_list.into_iter().enumerate() {
      result.push((format!("ShipmentStatusList.member.{}", i + 1), s.to_string()));
    }

    for (i, id) in self.shipment_id_list.into_iter().enumerate() {
      result.push((format!("ShipmentIdList.member.{}", i + 1), id.to_string()));
    }

    if let Some(date) = self.last_updated_after {
      result.push(("LastUpdatedAfter".to_string(), date.to_iso8601()));
    }

    if let Some(date) = self.last_updated_before {
      result.push(("LastUpdatedBefore".to_string(), date.to_iso8601()));
    }

    result
  }
}

#[derive(Debug, Default, PartialEq)]
pub struct ListInboundShipmentsResponse {
  pub request_id: String,
  pub shipment_data: Vec<InboundShipmentInfo>,
  pub next_token: Option<String>,
}

impl<S: decode::XmlEventStream> decode::FromXMLStream<S> for ListInboundShipmentsResponse {
  fn from_xml(s: &mut S) -> decode::Result<ListInboundShipmentsResponse> {
    use self::decode::{start_document, element, fold_elements, characters};
    start_document(s)?;
    element(s, vec!["ListInboundShipmentsResponse", "ListInboundShipmentsByNextTokenResponse"], |s| {
      fold_elements(s, ListInboundShipmentsResponse::default(), |s, response| {
        match s.local_name() {
          "ListInboundShipmentsResult" | "ListInboundShipmentsByNextTokenResult" => {
            fold_elements(s, (), |s, _| {
              match s.local_name() {
                "ShipmentData" => {
                  response.shipment_data = fold_elements(s, vec![], |s, list| {
                    list.push(InboundShipmentInfo::from_xml(s)?);
                    Ok(())
                  })?;
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

/// Returns a list of inbound shipments based on criteria that you specify.
/// 
/// [Documentation](http://docs.developer.amazonservices.com/en_CA/fba_inbound/FBAInbound_ListInboundShipments.html)
#[allow(non_snake_case)]
pub fn ListInboundShipments(client: &Client, parameters: ListInboundShipmentsParameters) -> Result<Response<ListInboundShipmentsResponse>> {
  client.request_xml(Method::Post, PATH, VERSION, "ListInboundShipments", parameters).map_err(|err| err.into())
}

/// Returns the next page of inbound shipments using the NextToken parameter.
///
/// [Documentation](http://docs.developer.amazonservices.com/en_CA/fba_inbound/FBAInbound_ListInboundShipmentsByNextToken.html)
#[allow(non_snake_case)]
pub fn ListInboundShipmentsByNextToken(client: &Client, next_token: String) -> Result<Response<ListInboundShipmentsResponse>> {
  let params = vec![
    ("NextToken".to_string(), next_token)
  ]; 
  client.request_xml(Method::Post, PATH, VERSION, "ListInboundShipmentsByNextToken", params).map_err(|err| err.into())
}

/// Parameters for `ListInboundShipments`
#[derive(Debug, Default)]
pub struct ListInboundShipmentItemsParameters {
  pub shipment_id: String,
  pub last_updated_after: Option<DateTime<UTC>>,
  pub last_updated_before: Option<DateTime<UTC>>,
}

impl Into<Vec<(String, String)>> for ListInboundShipmentItemsParameters {
  fn into(self) -> Vec<(String, String)> {
    let mut result = vec![
      ("ShipmentId".to_owned(), self.shipment_id)
    ];

    if let Some(date) = self.last_updated_after {
      result.push(("LastUpdatedAfter".to_string(), date.to_iso8601()));
    }

    if let Some(date) = self.last_updated_before {
      result.push(("LastUpdatedBefore".to_string(), date.to_iso8601()));
    }

    result
  }
}

#[derive(Debug, Default, PartialEq)]
pub struct ListInboundShipmentItemsResponse {
  pub request_id: String,
  pub item_data: Vec<InboundShipmentItem>,
  pub next_token: Option<String>,
}

impl<S: decode::XmlEventStream> decode::FromXMLStream<S> for ListInboundShipmentItemsResponse {
  fn from_xml(s: &mut S) -> decode::Result<ListInboundShipmentItemsResponse> {
    use self::decode::{start_document, element, fold_elements, characters};
    start_document(s)?;
    element(s, vec!["ListInboundShipmentItemsResponse", "ListInboundShipmentItemsByNextTokenResponse"], |s| {
      fold_elements(s, ListInboundShipmentItemsResponse::default(), |s, response| {
        match s.local_name() {
          "ListInboundShipmentItemsResult" | "ListInboundShipmentItemsByNextTokenResult" => {
            fold_elements(s, (), |s, _| {
              match s.local_name() {
                "ItemData" => {
                  response.item_data = fold_elements(s, vec![], |s, list| {
                    list.push(InboundShipmentItem::from_xml(s)?);
                    Ok(())
                  })?;
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

/// Returns a list of items in a specified inbound shipment, or a list of items that were updated within a specified time frame.
/// 
/// [Documentation](http://docs.developer.amazonservices.com/en_CA/fba_inbound/FBAInbound_ListInboundShipmentItems.html)
#[allow(non_snake_case)]
pub fn ListInboundShipmentItems(client: &Client, parameters: ListInboundShipmentItemsParameters) -> Result<Response<ListInboundShipmentItemsResponse>> {
  client.request_xml(Method::Post, PATH, VERSION, "ListInboundShipmentItems", parameters).map_err(|err| err.into())
}

/// Returns the next page of inbound shipment items using the NextToken parameter.
///
/// [Documentation](http://docs.developer.amazonservices.com/en_CA/fba_inbound/FBAInbound_ListInboundShipmentItemsByNextToken.html)
#[allow(non_snake_case)]
pub fn ListInboundShipmentItemsByNextToken(client: &Client, next_token: String) -> Result<Response<ListInboundShipmentItemsResponse>> {
  let params = vec![
    ("NextToken".to_string(), next_token)
  ]; 
  client.request_xml(Method::Post, PATH, VERSION, "ListInboundShipmentItemsByNextToken", params).map_err(|err| err.into())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_decode_list_inbound_shipments_response() {
    test_decode!(ListInboundShipmentsResponse, r#"
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
    "#, ListInboundShipmentsResponse {
      next_token: None,
      request_id: "04c87e79-f747-4da9-984f-5bc1f0b875e6".to_owned(),
      shipment_data: vec![
        InboundShipmentInfo {
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
        }
      ],
    });
  }

  #[test]
  fn test_decode_list_inbound_shipment_items_response() {
    test_decode!(ListInboundShipmentItemsResponse, r#"
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
    "#, ListInboundShipmentItemsResponse {
      next_token: None,
      request_id: "70a60f01-d0df-4a29-b093-e1cb53bd8fc2".to_owned(),
      item_data: vec![
        InboundShipmentItem {
          ShipmentId: "FBA3T68MQL".to_owned(),
          SellerSKU: "edifier-r1280t-fba".to_owned(),
          QuantityShipped: 60,
          QuantityInCase: Some(2),
          QuantityReceived: Some(50),
          FulfillmentNetworkSKU: "B016P9HJIA".to_owned(),
        }
      ],
    });
  }

}