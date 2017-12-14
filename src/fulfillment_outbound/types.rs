use chrono::{DateTime, Utc};
use xmlhelper::decode;

/// The FulfillmentPolicy value that you chose when
/// you submitted the CreateFulfillmentOrder operation.
str_enum! {
  pub enum FulfillmentPolicy {
    FillOrKill,
    FillAll,
    FillAllAvailable,
  }
}

/// The current status of the fulfillment order.
str_enum! {
  pub enum FulfillmentOrderStatus {
    RECEIVED,
    INVALID,
    PLANNING,
    PROCESSING,
    CANCELLED,
    COMPLETE,
    COMPLETE_PARTIALLED,
    UNFULFILLABLE,
  }
}

#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq)]
pub struct FulfillmentOrder {
  pub SellerFulfillmentOrderId: String,
  pub DestinationAddress: DestinationAddress,
  pub DisplayableOrderDateTime: Option<DateTime<Utc>>,
  pub ShippingSpeedCategory: String,
  pub FulfillmentMethod: String,
  pub FulfillmentOrderStatus: FulfillmentOrderStatus,
  pub StatusUpdatedDateTime: Option<DateTime<Utc>>,
  pub FulfillmentPolicy: FulfillmentPolicy,
  pub ReceivedDateTime: Option<DateTime<Utc>>,
  pub DisplayableOrderId: String,
  pub DisplayableOrderComment: String,
  pub MarketplaceId: Option<String>,
  pub FulfillmentAction: Option<String>,
  pub NotificationEmailList: Vec<String>,
}

impl<S: decode::XmlEventStream> decode::FromXMLStream<S> for FulfillmentOrder {
  fn from_xml(s: &mut S) -> decode::Result<FulfillmentOrder> {
    use xmlhelper::decode::{fold_elements, characters};
    fold_elements(s, FulfillmentOrder::default(), |s, record| {
      match s.local_name() {
        "SellerFulfillmentOrderId" => record.SellerFulfillmentOrderId = characters(s)?,
        "DestinationAddress" => record.DestinationAddress = DestinationAddress::from_xml(s)?,
        "DisplayableOrderDateTime" => record.DisplayableOrderDateTime = characters(s).map(Some)?,
        "ShippingSpeedCategory" => record.ShippingSpeedCategory = characters(s)?,
        "FulfillmentMethod" => record.FulfillmentMethod = characters(s)?,
        "FulfillmentOrderStatus" => record.FulfillmentOrderStatus = characters(s)?,
        "StatusUpdatedDateTime" => record.StatusUpdatedDateTime = characters(s).map(Some)?,
        "FulfillmentPolicy" => record.FulfillmentPolicy = characters(s)?,
        "ReceivedDateTime" => record.ReceivedDateTime = characters(s).map(Some)?,
        "DisplayableOrderId" => record.DisplayableOrderId = characters(s)?,
        "DisplayableOrderComment" => record.DisplayableOrderComment = characters(s)?,
        "MarketplaceId" => record.MarketplaceId = characters(s).map(Some)?,
        "FulfillmentAction" => record.FulfillmentAction = characters(s).map(Some)?,
        "NotificationEmailList" => {
          record.NotificationEmailList = fold_elements(s, vec![], |s, v| {
            v.push(characters(s)?);
            Ok(())
          })?;
        }
        _ => {}
      }
      Ok(())
    })
  }
}

#[allow(non_snake_case)]
#[derive(Debug, Default)]
pub struct Currency {
  /// Three-digit currency code.
  pub CurrencyCode: String,
  /// The currency amount.
  pub Value: String,
}

impl<S: decode::XmlEventStream> decode::FromXMLStream<S> for Currency {
  fn from_xml(s: &mut S) -> decode::Result<Currency> {
    use xmlhelper::decode::{fold_elements, characters};
    fold_elements(s, Currency::default(), |s, record| {
      match s.local_name() {
        "CurrencyCode" => record.CurrencyCode = characters(s)?,
        "Value" => record.Value = characters(s)?,
        _ => {}
      }
      Ok(())
    })
  }
}

#[allow(non_snake_case)]
#[derive(Debug, Default)]
pub struct FulfillmentOrderItem {
  /// The seller SKU of the item.
  pub SellerSKU: String,
  /// The fulfillment order item identifier that you created and
  /// submitted with the CreateFulfillmentOrder operation.
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
  /// The item quantity that was cancelled by the seller.
  pub CancelledQuantity: i32,
  /// The item quantity that is unfulfillable.
  pub UnfulfillableQuantity: i32,
  /// The estimated time that the item quantity is scheduled to
  /// ship from the fulfillment center. Note that this value can
  /// change over time. If the shipment that contains the item
  /// quantity has been cancelled, EstimatedShipDateTime is not
  /// returned.
  pub EstimatedShipDateTime: Option<DateTime<Utc>>,
  /// The estimated arrival time of the item quantity, . Note
  /// that this value can change over time. If the shipment that
  /// contains the item quantity has been cancelled,
  /// EstimatedArrivalDateTime is not returned.
  pub EstimatedArrivalDateTime: Option<DateTime<Utc>>,
  /// The monetary value assigned by the seller to this item.
  pub PerUnitDeclaredValue: Option<Currency>,
  /// The amount to be collected from the customer for this
  /// item in a COD (Cash On Delivery) order.
  /// Note: COD fulfillment orders are available only in
  /// China (CN) and Japan (JP).
  pub PerUnitPrice: Option<Currency>,
  /// The tax on the amount to be collected from the customer
  /// for this item in a COD (Cash On Delivery) order.
  /// Note: COD fulfillment orders are available only in
  /// CN and JP.
  pub PerUnitTax: Option<Currency>,
}

impl<S: decode::XmlEventStream> decode::FromXMLStream<S> for FulfillmentOrderItem {
  fn from_xml(s: &mut S) -> decode::Result<FulfillmentOrderItem> {
    use xmlhelper::decode::{fold_elements, characters};

    fold_elements(s, FulfillmentOrderItem::default(), |s, record| {
      match s.local_name() {
        "SellerSKU" => record.SellerSKU = characters(s)?,
        "SellerFulfillmentOrderItemId" => record.SellerFulfillmentOrderItemId = characters(s)?,
        "Quantity" => record.Quantity = characters(s)?,
        "GiftMessage" => record.GiftMessage = characters(s).map(Some)?,
        "DisplayableComment" => record.DisplayableComment = characters(s).map(Some)?,
        "FulfillmentNetworkSKU" => record.FulfillmentNetworkSKU = characters(s).map(Some)?,
        "CancelledQuantity" => record.CancelledQuantity = characters(s)?,
        "UnfulfillableQuantity" => record.UnfulfillableQuantity = characters(s)?,
        "EstimatedShipDateTime" => record.EstimatedShipDateTime = characters(s).map(Some)?,
        "PerUnitDeclaredValue" => record.PerUnitDeclaredValue = Currency::from_xml(s).map(Some)?,
        "PerUnitPrice" => record.PerUnitPrice = Currency::from_xml(s).map(Some)?,
        "PerUnitTax" => record.PerUnitTax = Currency::from_xml(s).map(Some)?,
        _ => {}
      }
      Ok(())
    })
  }
}

#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq)]
pub struct DestinationAddress {
  pub PhoneNumber: String,
  pub City: String,
  pub CountryCode: String,
  pub PostalCode: String,
  pub Name: String,
  pub StateOrProvinceCode: String,
  pub DistrictOrCounty: String,
  pub Line1: String,
  pub Line2: String,
  pub Line3: String,
}

impl<S: decode::XmlEventStream> decode::FromXMLStream<S> for DestinationAddress {
  fn from_xml(s: &mut S) -> decode::Result<DestinationAddress> {
    use xmlhelper::decode::{fold_elements, characters};
    fold_elements(s, DestinationAddress::default(), |s, record| {
      match s.local_name() {
        "PhoneNumber" => record.PhoneNumber = characters(s)?,
        "City" => record.City = characters(s)?,
        "CountryCode" => record.CountryCode = characters(s)?,
        "PostalCode" => record.PostalCode = characters(s)?,
        "StateOrProvinceCode" => record.StateOrProvinceCode = characters(s)?,
        "DistrictOrCounty" => record.DistrictOrCounty = characters(s)?,
        "Name" => record.Name = characters(s)?,
        "Line1" => record.Line1 = characters(s)?,
        "Line2" => record.Line2 = characters(s)?,
        "Line3" => record.Line3 = characters(s)?,
        _ => {}
      }
      Ok(())
    })
  }
}

#[derive(Debug, Default)]
pub struct ReturnItemList;

#[derive(Debug, Default)]
pub struct ReturnAuthorizationList;

/// The current status of the shipment.
str_enum! {
  pub enum FulfillmentShipmentStatus {
    PENDING,
    SHIPPED,
    CANCELLED_BY_FULFILLER,
    CANCELLED_BY_SELLER,
  }
}

/// Delivery and item information for a shipment in a fulfillment order.
#[allow(non_snake_case)]
#[derive(Debug, Default)]
pub struct FulfillmentShipment {
  /// A shipment identifier assigned by Amazon.
  pub AmazonShipmentId: String,
  /// An identifier for the Amazon fulfillment center
  /// that the shipment will be sent from.
  pub FulfillmentCenterId: String,
  /// The current status of the shipment.
  pub FulfillmentShipmentStatus: FulfillmentShipmentStatus,
  /// The meaning of the ShippingDateTime value
  /// depends on the current status of the shipment. If
  /// the current value of FulfillmentShipmentStatus is:
  /// - Pending - ShippingDateTime represents the
  /// estimated time that the shipment will leave the Amazon fulfillment center.
  /// - Shipped - ShippingDateTime represents the
  /// date that the shipment left the Amazon fulfillment center.
  /// If a shipment includes more than one package,
  /// ShippingDateTime applies to all of the packages in
  /// the shipment. If the value of FulfillmentShipmentStatus
  /// is CancelledByFulfiller or CancelledBySeller, ShippingDateTime
  /// is not returned.
  pub ShippingDateTime: Option<DateTime<Utc>>,
  /// The estimated arrival time of the shipment. Note
  /// that this value can change over time. If a shipment
  /// includes more than one package, EstimatedArrivalDateTime
  /// applies to all of the packages in the shipment. If the
  /// shipment has been cancelled, EstimatedArrivalDateTime
  /// is not returned.
  pub EstimatedArrivalDateTime: Option<DateTime<Utc>>,
  /// Information about the items in the shipment.
  pub FulfillmentShipmentItem: Vec<FulfillmentShipmentItem>,
  /// Information about a single package in the shipment.
  pub FulfillmentShipmentPackage: Vec<FulfillmentShipmentPackage>,
}

impl<S: decode::XmlEventStream> decode::FromXMLStream<S> for FulfillmentShipment {
  fn from_xml(s: &mut S) -> decode::Result<FulfillmentShipment> {
    use xmlhelper::decode::{fold_elements, characters};
    fold_elements(s, FulfillmentShipment::default(), |s, record| {
      match s.local_name() {
        "AmazonShipmentId" => record.AmazonShipmentId = characters(s)?,
        "FulfillmentCenterId" => record.FulfillmentCenterId = characters(s)?,
        "FulfillmentShipmentStatus" => record.FulfillmentShipmentStatus = characters(s)?,
        "ShippingDateTime" => record.ShippingDateTime = characters(s).map(Some)?,
        "EstimatedArrivalDateTime" => record.EstimatedArrivalDateTime = characters(s).map(Some)?,
        "FulfillmentShipmentItem" => {
          record.FulfillmentShipmentItem = fold_elements(s, vec![], |s, v| {
            v.push(FulfillmentShipmentItem::from_xml(s)?);
            Ok(())
          })?
        }
        "FulfillmentShipmentPackage" => {
          record.FulfillmentShipmentPackage = fold_elements(s, vec![], |s, v| {
            v.push(FulfillmentShipmentPackage::from_xml(s)?);
            Ok(())
          })?
        }
        _ => {}
      }
      Ok(())
    })
  }
}

/// Item information for a shipment in a fulfillment order.
#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq)]
pub struct FulfillmentShipmentItem {
  /// The seller SKU of the item.
  pub SellerSKU: Option<String>,
  ///	The fulfillment order item identifier that you created
  /// and submitted with the CreateFulfillmentOrder operation.
  pub SellerFulfillmentOrderItemId: String,
  /// The item quantity.
  pub Quantity: i32,
  /// An identifier for the package that contains the item
  /// quantity.
  pub PackageNumber: Option<String>,
}

impl<S: decode::XmlEventStream> decode::FromXMLStream<S> for FulfillmentShipmentItem {
  fn from_xml(s: &mut S) -> decode::Result<FulfillmentShipmentItem> {
    use xmlhelper::decode::{fold_elements, characters};
    fold_elements(s, FulfillmentShipmentItem::default(), |s, record| {
      match s.local_name() {
        "SellerSKU" => record.SellerSKU = characters(s).map(Some)?,
        "SellerFulfillmentOrderItemId" => record.SellerFulfillmentOrderItemId = characters(s)?,
        "Quantity" => record.Quantity = characters(s)?,
        "PackageNumber" => record.PackageNumber = characters(s).map(Some)?,
        _ => {}
      }
      Ok(())
    })
  }
}

#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq)]
pub struct FulfillmentShipmentPackage {
  /// Identifies a package in a shipment.
  pub PackageNumber: String,
  /// Identifies the carrier who will deliver the
  /// shipment to the recipient.
  pub CarrierCode: String,
  /// The tracking number, if provided, can be used to
  /// obtain tracking and delivery information.
  pub TrackingNumber: Option<String>,
  /// The estimated arrival time of the package.
  pub EstimatedArrivalDateTime: Option<DateTime<Utc>>,
}

impl<S: decode::XmlEventStream> decode::FromXMLStream<S> for FulfillmentShipmentPackage {
  fn from_xml(s: &mut S) -> decode::Result<FulfillmentShipmentPackage> {
    use xmlhelper::decode::{fold_elements, characters};
    fold_elements(s, FulfillmentShipmentPackage::default(), |s, record| {
      match s.local_name() {
        "PackageNumber" => record.PackageNumber = characters(s)?,
        "CarrierCode" => record.CarrierCode = characters(s)?,
        "TrackingNumber" => record.TrackingNumber = characters(s).map(Some)?,
        "EstimatedArrivalDateTime" => record.EstimatedArrivalDateTime = characters(s).map(Some)?,
        _ => {}
      }
      Ok(())
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use xmlhelper::decode;
  use xmlhelper::decode::FromXMLStream;
  use std::io::Cursor;
  use chrono::TimeZone;

  #[test]
  fn test_decode_destination_address() {
    test_decode!(
      DestinationAddress,
      r#"
        <PhoneNumber>(111) 222-3333</PhoneNumber>
        <City>DORADO</City>
        <CountryCode>US</CountryCode>
        <PostalCode>888888</PostalCode>
        <Name>Foo Bar</Name>
        <StateOrProvinceCode>CA</StateOrProvinceCode>
        <Line3/>
        <Line2/>
        <DistrictOrCounty/>
        <Line1>1818 DRIFTWOOD CIRCLE</Line1>
      "#,
      DestinationAddress {
        PhoneNumber: "(111) 222-3333".to_owned(),
        City: "DORADO".to_owned(),
        CountryCode: "US".to_owned(),
        PostalCode: "888888".to_owned(),
        Name: "Foo Bar".to_owned(),
        StateOrProvinceCode: "CA".to_owned(),
        DistrictOrCounty: "".to_owned(),
        Line1: "1818 DRIFTWOOD CIRCLE".to_owned(),
        Line2: "".to_owned(),
        Line3: "".to_owned(),
      }
    );
  }

  #[test]
  fn test_fulfillment_order() {
    test_decode!(
      FulfillmentOrder,
      r#"
        <SellerFulfillmentOrderId>11111111</SellerFulfillmentOrderId>
        <DestinationAddress>
          <PhoneNumber>22222222</PhoneNumber>
          <City>City Name</City>
          <CountryCode>US</CountryCode>
          <PostalCode>333333</PostalCode>
          <Name>Foo Bar</Name>
          <StateOrProvinceCode>TX</StateOrProvinceCode>
          <Line3/>
          <DistrictOrCounty/>
          <Line2/>
          <Line1>2907 Switch Case St</Line1>
        </DestinationAddress>
        <DisplayableOrderDateTime>2017-12-11T08:00:00Z</DisplayableOrderDateTime>
        <ShippingSpeedCategory>Expedited</ShippingSpeedCategory>
        <FulfillmentMethod>Consumer</FulfillmentMethod>
        <FulfillmentOrderStatus>COMPLETE</FulfillmentOrderStatus>
        <FulfillmentPolicy>FillOrKill</FulfillmentPolicy>
        <StatusUpdatedDateTime>2017-12-12T10:27:43Z</StatusUpdatedDateTime>
        <MarketplaceId>ATVPDKIKX0DER</MarketplaceId>
        <ReceivedDateTime>2017-12-13T10:27:43Z</ReceivedDateTime>
        <FulfillmentAction>Ship</FulfillmentAction>
        <NotificationEmailList>
          <member>hello@ventmere.com</member>
        </NotificationEmailList>
        <DisplayableOrderId>55555555</DisplayableOrderId>
        <DisplayableOrderComment>Thank you for your order!</DisplayableOrderComment>
      "#,
      FulfillmentOrder {
        SellerFulfillmentOrderId: "11111111".to_owned(),
        DestinationAddress: DestinationAddress {
          PhoneNumber: "22222222".to_owned(),
          City: "City Name".to_owned(),
          CountryCode: "US".to_owned(),
          PostalCode: "333333".to_owned(),
          Name: "Foo Bar".to_owned(),
          StateOrProvinceCode: "TX".to_owned(),
          DistrictOrCounty: "".to_owned(),
          Line1: "2907 Switch Case St".to_owned(),
          Line2: "".to_owned(),
          Line3: "".to_owned(),
        },
        DisplayableOrderDateTime: Some(Utc.ymd(2017, 12, 11).and_hms(8, 0, 0)),
        ShippingSpeedCategory: "Expedited".to_owned(),
        FulfillmentMethod: "Consumer".to_owned(),
        FulfillmentOrderStatus: FulfillmentOrderStatus::COMPLETE,
        StatusUpdatedDateTime: Some(Utc.ymd(2017, 12, 12).and_hms(10, 27, 43)),
        FulfillmentPolicy: FulfillmentPolicy::FillOrKill,
        ReceivedDateTime: Some(Utc.ymd(2017, 12, 13).and_hms(10, 27, 43)),
        DisplayableOrderId: "55555555".to_owned(),
        DisplayableOrderComment: "Thank you for your order!".to_owned(),
        MarketplaceId: Some("ATVPDKIKX0DER".to_owned()),
        FulfillmentAction: Some("Ship".to_owned()),
        NotificationEmailList: vec!["hello@ventmere.com".to_owned()],
      }
    );

  }

  #[test]
  fn test_fulfillment_shipment_item() {
    test_decode!(
      FulfillmentShipmentItem,
      r#"
        <SellerFulfillmentOrderItemId>e-1111</SellerFulfillmentOrderItemId>
        <Quantity>1</Quantity>
        <SellerSKU>e-1111</SellerSKU>
        <PackageNumber>1899999</PackageNumber>
      "#,
      FulfillmentShipmentItem {
        SellerFulfillmentOrderItemId: "e-1111".to_owned(),
        Quantity: 1,
        SellerSKU: Some("e-1111".to_owned()),
        PackageNumber: Some("1899999".to_owned()),
      }
    );
  }

  #[test]
  fn test_fulfillment_shipment_package() {
    test_decode!(
      FulfillmentShipmentPackage,
      r#"
        <EstimatedArrivalDateTime>2017-12-03T04:00:00Z</EstimatedArrivalDateTime>
        <TrackingNumber>7777777777</TrackingNumber>
        <CarrierCode>USPS</CarrierCode>
        <PackageNumber>185528000</PackageNumber>
      "#,
      FulfillmentShipmentPackage {
        EstimatedArrivalDateTime: Some(Utc.ymd(2017, 12, 3).and_hms(4, 0, 0)),
        TrackingNumber: Some("7777777777".to_owned()),
        CarrierCode: "USPS".to_owned(),
        PackageNumber: "185528000".to_owned(),
      }
    );
  }
}