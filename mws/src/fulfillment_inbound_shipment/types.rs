str_enum! {
  pub enum ShipmentStatus  {
    WORKING,    // - The shipment was created by the seller, but has not yet shipped.
    SHIPPED,    // - The shipment was picked up by the carrier.
    IN_TRANSIT, // - The carrier has notified the Amazon fulfillment center that it is aware of the shipment.
    DELIVERED,  // - The shipment was delivered by the carrier to the Amazon fulfillment center.
    CHECKED_IN, // - The shipment was checked-in at the receiving dock of the Amazon fulfillment center.
    RECEIVING,  // - The shipment has arrived at the Amazon fulfillment center, but not all items have been marked as received.
    CLOSED,     // - The shipment has arrived at the Amazon fulfillment center and all items have been marked as received.
    CANCELLED,  // - The shipment was cancelled by the seller after the shipment was sent to the Amazon fulfillment center.
    DELETED,    // - The shipment was cancelled by the seller before the shipment was sent to the Amazon fulfillment center.
    ERROR,      // - There was an error with the shipment and it was not processed by Amazon.
  }
}

string_map_enum! {
  /// Where the seller provided box contents information for a shipment. This is only returned for shipments to US fulfillment centers.
  pub enum BoxContentsSource {
    NONE = "NONE",
    FEED = "FEED",
    _2D_BARCODE = "2D_BARCODE",
    INTERACTIVE = "INTERACTIVE",
  }
}

#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq, Serialize, FromXmlStream)]
pub struct Amount {
  pub CurrencyCode: String,
  pub Value: String,
}

/// The manual processing fee per unit and total fee for a shipment.
#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq, Serialize, FromXmlStream)]
pub struct BoxContentsFeeDetails {
  pub TotalUnits: i32,
  pub FeePerUnit: Option<Amount>,
  pub TotalFee: Option<Amount>,
}

str_enum! {
  pub enum LabelPrepType {
    NO_LABEL,
    SELLER_LABEL,
    AMAZON_LABEL,
  }
}

/// Postal address information.
#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq, Serialize, FromXmlStream)]
pub struct Address {
  /// The name or business name.
  pub Name: String,
  /// The street address information.
  pub AddressLine1: String,
  ///	Additional street address information, if required.
  pub AddressLine2: String,
  /// The city.
  pub City: String,
  /// The district or county.
  pub DistrictOrCounty: String,
  /// The state or province code.
  pub StateOrProvinceCode: String,
  /// The country code.
  pub CountryCode: String,
  /// The postal code.
  pub PostalCode: String,
}

/// Information about your inbound shipments. Returned by the `ListInboundShipments` operation.
#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq, Serialize, FromXmlStream)]
pub struct InboundShipmentInfo {
  /// The ShipmentId submitted in the request.
  pub ShipmentId: String,
  /// A unique name that you provide for your inbound shipment.
  pub ShipmentName: String,
  /// Your return address.
  pub ShipFromAddress: Address,
  /// An Amazon fulfillment center identifier created by Amazon.
  pub DestinationFulfillmentCenterId: String,
  /// The type of label preparation that is required for your inbound shipment.
  pub LabelPrepType: Option<LabelPrepType>,
  /// The status of your inbound shipment.
  pub ShipmentStatus: ShipmentStatus,
  /// Indicates whether or not an inbound shipment contains case-packed boxes.
  /// When AreCasesRequired = true for an inbound shipment, all items in the
  /// inbound shipment must be case packed.
  pub AreCasesRequired: bool,
  ///	Date that the shipment must arrive at an Amazon fulfillment center to avoid
  /// delivery promise breaks for pre-ordered items. For more information, see
  /// GetPreorderInfo. Pre-orders are only available in India and Japan.
  pub ConfirmedNeedByDate: Option<String>,
  ///	Where the seller provided box contents information for a shipment. This is
  /// only returned for shipments to US fulfillment centers.
  pub BoxContentsSource: Option<BoxContentsSource>,
  /// An estimate of the manual processing fee charged by Amazon for boxes
  /// without box content information. This is only returned when BoxContentsSource is NONE.
  pub EstimatedBoxContentsFee: Option<BoxContentsFeeDetails>,
}

/// Item information for an inbound shipment. Submitted with a call to the CreateInboundShipment or UpdateInboundShipment operation.
#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq, Serialize, FromXmlStream)]
pub struct InboundShipmentItem {
  pub ShipmentId: String,
  pub SellerSKU: String,
  pub FulfillmentNetworkSKU: String,
  pub QuantityShipped: i32,
  pub QuantityReceived: Option<i32>,
  pub QuantityInCase: Option<i32>,
  // PrepDetailsList: List of PrepDetails
  // ReleaseDate: xs:string,
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::io::Cursor;
  use xmlhelper::decode;
  use xmlhelper::decode::FromXmlStream;

  #[test]
  fn test_decode_inbound_shipment_info() {
    let mut s = decode::Stream::new(Cursor::new(
      r#"
    <member>
        <ShipFromAddress>
            <PostalCode>V5V 1A1</PostalCode>
            <Name>Janani Arvind FBA QA</Name>
            <CountryCode>CA</CountryCode>
            <StateOrProvinceCode>BC</StateOrProvinceCode>
            <AddressLine1>Address 1</AddressLine1>
            <City>Vancouver</City>
        </ShipFromAddress>
        <ShipmentId>FBA1123</ShipmentId>
        <ShipmentName>Test MWS CA Shipment 1</ShipmentName>
        <ShipmentStatus>WORKING</ShipmentStatus>
        <LabelPrepType>NO_LABEL</LabelPrepType>
        <DestinationFulfillmentCenterId>RIC2</DestinationFulfillmentCenterId>
        <BoxContentsSource>NONE</BoxContentsSource>
        <EstimatedBoxContentsFee>
            <TotalUnits>10</TotalUnits>
            <FeePerUnit>
                <CurrencyCode>USD</CurrencyCode>
                <Value>0.10</Value>
            </FeePerUnit>
            <TotalFee>
                <CurrencyCode>USD</CurrencyCode>
                <Value>10.0</Value>
            </TotalFee>
        </EstimatedBoxContentsFee>
    </member>
    "#,
    ));

    decode::start_document(&mut s).expect("start element");
    let record = decode::element(&mut s, "member", |s| InboundShipmentInfo::from_xml(s)).unwrap();
    assert_eq!(
      record,
      InboundShipmentInfo {
        ShipFromAddress: Address {
          PostalCode: "V5V 1A1".to_owned(),
          Name: "Janani Arvind FBA QA".to_owned(),
          CountryCode: "CA".to_owned(),
          StateOrProvinceCode: "BC".to_owned(),
          AddressLine1: "Address 1".to_owned(),
          AddressLine2: "".to_owned(),
          City: "Vancouver".to_owned(),
          DistrictOrCounty: "".to_owned(),
        },
        ShipmentId: "FBA1123".to_owned(),
        ShipmentName: "Test MWS CA Shipment 1".to_owned(),
        ShipmentStatus: ShipmentStatus::WORKING,
        LabelPrepType: Some(LabelPrepType::NO_LABEL),
        DestinationFulfillmentCenterId: "RIC2".to_owned(),
        BoxContentsSource: Some(BoxContentsSource::NONE),
        EstimatedBoxContentsFee: Some(BoxContentsFeeDetails {
          TotalUnits: 10,
          FeePerUnit: Some(Amount {
            CurrencyCode: "USD".to_owned(),
            Value: "0.10".to_owned(),
          }),
          TotalFee: Some(Amount {
            CurrencyCode: "USD".to_owned(),
            Value: "10.0".to_owned(),
          })
        }),
        AreCasesRequired: false,
        ConfirmedNeedByDate: None,
      }
    );
  }

  #[test]
  fn test_decode_inbound_shipment_item() {
    let mut s = decode::Stream::new(Cursor::new(
      r#"
    <member>
        <ShipmentId>SSF85DGIZZ3OF1</ShipmentId>
        <SellerSKU>SampleSKU2</SellerSKU>
        <QuantityShipped>10</QuantityShipped>
        <QuantityInCase>0</QuantityInCase>
        <QuantityReceived>0</QuantityReceived>
        <FulfillmentNetworkSKU>B0011VECH4</FulfillmentNetworkSKU>
    </member>
    "#,
    ));

    decode::start_document(&mut s).expect("start element");
    let record = decode::element(&mut s, "member", |s| InboundShipmentItem::from_xml(s)).unwrap();

    assert_eq!(
      record,
      InboundShipmentItem {
        ShipmentId: "SSF85DGIZZ3OF1".to_owned(),
        SellerSKU: "SampleSKU2".to_owned(),
        QuantityShipped: 10,
        QuantityInCase: Some(0),
        QuantityReceived: Some(0),
        FulfillmentNetworkSKU: "B0011VECH4".to_owned(),
      }
    );
  }
}
