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
    use xmlhelper::decode::{characters, fold_elements};
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
#[derive(Debug, Default, PartialEq)]
pub struct Currency {
  /// Three-digit currency code.
  pub CurrencyCode: String,
  /// The currency amount.
  pub Value: String,
}

impl<S: decode::XmlEventStream> decode::FromXMLStream<S> for Currency {
  fn from_xml(s: &mut S) -> decode::Result<Currency> {
    use xmlhelper::decode::{characters, fold_elements};
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
    use xmlhelper::decode::{characters, fold_elements};

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
    use xmlhelper::decode::{characters, fold_elements};
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
    use xmlhelper::decode::{characters, fold_elements};
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
    use xmlhelper::decode::{characters, fold_elements};
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
    use xmlhelper::decode::{characters, fold_elements};
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

/// The current status of the fulfillment order.
str_enum! {
  pub enum PackageStatus {
    IN_TRANSIT,
    DELIVERED,
    RETURNING,
    RETURNED,
    UNDELIVERABLE,
    DELAYED,
    AVAILABLE_FOR_PICKUP,
    CUSTOMER_ACTION,
  }
}

#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq)]
pub struct TrackingAddress {
  pub City: String,
  pub State: String,
  pub Country: String,
}

impl<S: decode::XmlEventStream> decode::FromXMLStream<S> for TrackingAddress {
  fn from_xml(s: &mut S) -> decode::Result<TrackingAddress> {
    use xmlhelper::decode::{characters, fold_elements};
    fold_elements(s, TrackingAddress::default(), |s, record| {
      match s.local_name() {
        "City" => record.City = characters(s)?,
        "State" => record.State = characters(s)?,
        "Country" => record.Country = characters(s)?,
        _ => {}
      }
      Ok(())
    })
  }
}

#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq)]
pub struct TrackingEvent {
  pub EventDate: Option<DateTime<Utc>>,
  pub EventAddress: Option<TrackingAddress>,
  pub EventCode: String,
}

impl<S: decode::XmlEventStream> decode::FromXMLStream<S> for TrackingEvent {
  fn from_xml(s: &mut S) -> decode::Result<TrackingEvent> {
    use xmlhelper::decode::{characters, fold_elements};
    fold_elements(s, TrackingEvent::default(), |s, record| {
      match s.local_name() {
        "EventDate" => record.EventDate = characters(s).map(Some)?,
        "EventAddress" => record.EventAddress = TrackingAddress::from_xml(s).map(Some)?,
        "EventCode" => record.EventCode = characters(s)?,
        _ => {}
      }
      Ok(())
    })
  }
}

impl TrackingEvent {
  pub fn event_description(&self) -> Option<&'static str> {
    let value = match self.EventCode.as_ref() {
      "EVENT_101" => "Carrier notified to pick up package.",
      "EVENT_102" => "Shipment picked up from seller's facility.",
      "EVENT_201" => "Arrival scan.",
      "EVENT_202" => "Departure scan.",
      "EVENT_203" => "Arrived at destination country.",
      "EVENT_204" => "Initiated customs clearance process.",
      "EVENT_205" => "Completed customs clearance process.",
      "EVENT_206" => "In transit to pickup location.",
      "EVENT_301" => "Delivered.",
      "EVENT_302" => "Out for delivery.",
      "EVENT_304" => "Delivery attempted.",
      "EVENT_306" => "Customer contacted to arrange delivery.",
      "EVENT_307" => "Delivery appointment scheduled.",
      "EVENT_308" => "Available for pickup.",
      "EVENT_309" => "Returned to seller.",
      "EVENT_401" => "Held by carrier - incorrect address.",
      "EVENT_402" => "Customs clearance delay.",
      "EVENT_403" => "Customer moved.",
      "EVENT_404" => "Delay in delivery due to external factors.",
      "EVENT_405" => "Shipment damaged.",
      "EVENT_406" => "Held by carrier.",
      "EVENT_407" => "Customer refused delivery.",
      "EVENT_408" => "Returning to seller.",
      "EVENT_409" => "Lost by carrier.",
      "EVENT_411" => "Paperwork received - did not receive shipment.",
      "EVENT_412" => "Shipment received- did not receive paperwork.",
      "EVENT_413" => "Held by carrier- customer refused shipment due to customs charges.",
      "EVENT_414" => "Missorted by carrier.",
      "EVENT_415" => "Received from prior carrier.",
      "EVENT_416" => "Undeliverable.",
      "EVENT_417" => "Shipment missorted.",
      "EVENT_418" => "Shipment delayed.",
      "EVENT_419" => "Address corrected - delivery rescheduled.",
      _ => "",
    };
    if value.is_empty() {
      None
    } else {
      Some(value)
    }
  }
}

#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq)]
pub struct PackageTrackingDetails {
  /// The package identifier.
  pub PackageNumber: String,
  /// The tracking number for the package.
  pub TrackingNumber: Option<String>,
  /// The name of the carrier.
  pub CarrierCode: Option<String>,
  /// The phone number of the carrier.
  pub CarrierPhoneNumber: Option<String>,
  /// The URL of the carrier’s website.
  pub CarrierURL: Option<String>,
  /// The shipping date for the package.
  pub ShipDate: Option<DateTime<Utc>>,
  /// The destination city for the package.
  pub ShipToAddress: Option<TrackingAddress>,
  /// The current delivery status of the package.
  pub CurrentStatus: PackageStatus,
  /// The name of the person who signed for the package.
  pub SignedForBy: Option<String>,
  /// The estimated arrival date.
  pub EstimatedArrivalDate: Option<DateTime<Utc>>,
  /// A list of tracking events.
  pub TrackingEvents: Vec<TrackingEvent>,
  /// Additional location information.
  pub AdditionalLocationInfo: Option<String>,
}

impl<S: decode::XmlEventStream> decode::FromXMLStream<S> for PackageTrackingDetails {
  fn from_xml(s: &mut S) -> decode::Result<PackageTrackingDetails> {
    use xmlhelper::decode::{characters, fold_elements};
    fold_elements(s, PackageTrackingDetails::default(), |s, record| {
      match s.local_name() {
        "PackageNumber" => record.PackageNumber = characters(s)?,
        "TrackingNumber" => record.TrackingNumber = characters(s).map(Some)?,
        "CarrierCode" => record.CarrierCode = characters(s).map(Some)?,
        "CarrierPhoneNumber" => record.CarrierPhoneNumber = characters(s).map(Some)?,
        "CarrierURL" => record.CarrierURL = characters(s).map(Some)?,
        "ShipDate" => record.ShipDate = characters(s).map(Some)?,
        "ShipToAddress" => record.ShipToAddress = TrackingAddress::from_xml(s).map(Some)?,
        "CurrentStatus" => record.CurrentStatus = characters(s)?,
        "SignedForBy" => record.SignedForBy = characters(s).map(Some)?,
        "EstimatedArrivalDate" => record.EstimatedArrivalDate = characters(s).map(Some)?,
        "TrackingEvents" => {
          record.TrackingEvents = fold_elements(s, vec![], |s, v| {
            v.push(TrackingEvent::from_xml(s)?);
            Ok(())
          })?
        }
        "AdditionalLocationInfo" => record.AdditionalLocationInfo = characters(s).map(Some)?,
        _ => {}
      }
      Ok(())
    })
  }
}

/// Weight unit and amount.
#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq)]
pub struct Weight {
  /// Indicates the unit of weight.
  pub Unit: String,
  /// The numeric value of the item's weight.
  pub Value: String,
}

impl<S: decode::XmlEventStream> decode::FromXMLStream<S> for Weight {
  fn from_xml(s: &mut S) -> decode::Result<Weight> {
    use xmlhelper::decode::{characters, fold_elements};
    fold_elements(s, Weight::default(), |s, record| {
      match s.local_name() {
        "Unit" => record.Unit = characters(s)?,
        "Value" => record.Value = characters(s)?,
        _ => {}
      }
      Ok(())
    })
  }
}

/// Fee type and cost.
#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq)]
pub struct Fee {
  /// The type of fee.
  pub Name: String,
  /// The numeric value of the item's weight.
  pub Amount: Currency,
}

impl<S: decode::XmlEventStream> decode::FromXMLStream<S> for Fee {
  fn from_xml(s: &mut S) -> decode::Result<Fee> {
    use xmlhelper::decode::{characters, fold_elements};
    fold_elements(s, Fee::default(), |s, record| {
      match s.local_name() {
        "Name" => record.Name = characters(s)?,
        "Amount" => record.Amount = Currency::from_xml(s)?,
        _ => {}
      }
      Ok(())
    })
  }
}

/// Item information for a shipment in a fulfillment order preview.
#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq)]
pub struct FulfillmentPreviewItem {
  /// The seller SKU of the item.
  pub SellerSKU: String,
  /// A fulfillment order item identifier that you
  /// created with a call to the
  /// GetFulfillmentPreview operation.
  pub SellerFulfillmentOrderItemId: String,
  /// The item quantity.
  pub Quantity: i32,
  /// The estimated shipping weight of the item
  /// quantity for a single item, as identified by
  /// SellerSKU, in a shipment.
  pub EstimatedShippingWeight: Option<Weight>,
  /// 	The method used to calculate EstimatedShippingWeight.
  pub ShippingWeightCalculationMethod: Option<String>,
}

impl<S: decode::XmlEventStream> decode::FromXMLStream<S> for FulfillmentPreviewItem {
  fn from_xml(s: &mut S) -> decode::Result<FulfillmentPreviewItem> {
    use xmlhelper::decode::{characters, fold_elements};
    fold_elements(s, FulfillmentPreviewItem::default(), |s, record| {
      match s.local_name() {
        "SellerSKU" => record.SellerSKU = characters(s)?,
        "SellerFulfillmentOrderItemId" => record.SellerFulfillmentOrderItemId = characters(s)?,
        "Quantity" => record.Quantity = characters(s)?,
        "EstimatedShippingWeight" => {
          record.EstimatedShippingWeight = Weight::from_xml(s).map(Some)?
        }
        "ShippingWeightCalculationMethod" => {
          record.ShippingWeightCalculationMethod = characters(s).map(Some)?
        }
        _ => {}
      }
      Ok(())
    })
  }
}

/// Delivery and item information for a shipment in a fulfillment order preview.
#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq)]
pub struct FulfillmentPreviewShipment {
  /// The earliest date that the shipment is expected to
  /// be sent from the fulfillment center.
  pub EarliestShipDate: Option<DateTime<Utc>>,
  /// The latest date that the shipment is expected to be
  /// sent from the fulfillment center.
  pub LatestShipDate: Option<DateTime<Utc>>,
  /// The earliest date that the shipment is expected to
  /// arrive at its destination.
  pub EarliestArrivalDate: Option<DateTime<Utc>>,
  /// The latest date that the shipment is expected to
  /// arrive at its destination.
  pub LatestArrivalDate: Option<DateTime<Utc>>,
  /// Information about the items in the shipment.
  pub FulfillmentPreviewItems: Vec<FulfillmentPreviewItem>,
}

impl<S: decode::XmlEventStream> decode::FromXMLStream<S> for FulfillmentPreviewShipment {
  fn from_xml(s: &mut S) -> decode::Result<FulfillmentPreviewShipment> {
    use xmlhelper::decode::{characters, fold_elements};
    fold_elements(s, FulfillmentPreviewShipment::default(), |s, record| {
      match s.local_name() {
        "EarliestShipDate" => record.EarliestShipDate = characters(s).map(Some)?,
        "LatestShipDate" => record.LatestShipDate = characters(s).map(Some)?,
        "EarliestArrivalDate" => record.EarliestArrivalDate = characters(s).map(Some)?,
        "LatestArrivalDate" => record.LatestArrivalDate = characters(s).map(Some)?,
        "FulfillmentPreviewItems" => {
          record.FulfillmentPreviewItems = fold_elements(s, vec![], |s, v| {
            v.push(FulfillmentPreviewItem::from_xml(s)?);
            Ok(())
          })?
        }
        _ => {}
      }
      Ok(())
    })
  }
}

/// Information about unfulfillable items in a fulfillment order preview.
#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq)]
pub struct UnfulfillablePreviewItem {
  pub SellerSKU: String,
  pub SellerFulfillmentOrderItemId: String,
  pub Quantity: i32,
  /// Error codes associated with the fulfillment order
  /// preview that indicate why the item is unfulfillable.
  pub ItemUnfulfillableReasons: Option<Vec<String>>,
}

impl<S: decode::XmlEventStream> decode::FromXMLStream<S> for UnfulfillablePreviewItem {
  fn from_xml(s: &mut S) -> decode::Result<UnfulfillablePreviewItem> {
    use xmlhelper::decode::{characters, fold_elements};
    fold_elements(s, UnfulfillablePreviewItem::default(), |s, record| {
      match s.local_name() {
        "SellerSKU" => record.SellerSKU = characters(s)?,
        "SellerFulfillmentOrderItemId" => record.SellerFulfillmentOrderItemId = characters(s)?,
        "Quantity" => record.Quantity = characters(s)?,
        "ItemUnfulfillableReasons" => {
          record.ItemUnfulfillableReasons = fold_elements(s, vec![], |s, v| {
            v.push(characters(s)?);
            Ok(())
          }).map(Some)?
        }
        _ => {}
      }
      Ok(())
    })
  }
}

/// The time range within which your Scheduled Delivery fulfillment order should be delivered.
#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq)]
pub struct DeliveryWindow {
  pub StartDateTime: Option<DateTime<Utc>>,
  pub EndDateTime: Option<DateTime<Utc>>,
}

impl<S: decode::XmlEventStream> decode::FromXMLStream<S> for DeliveryWindow {
  fn from_xml(s: &mut S) -> decode::Result<DeliveryWindow> {
    use xmlhelper::decode::{characters, fold_elements};
    fold_elements(s, DeliveryWindow::default(), |s, record| {
      match s.local_name() {
        "StartDateTime" => record.StartDateTime = characters(s).map(Some)?,
        "EndDateTime" => record.EndDateTime = characters(s).map(Some)?,
        _ => {}
      }
      Ok(())
    })
  }
}

/// Delivery information for a Scheduled Delivery.
#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq)]
pub struct ScheduledDeliveryInfo {
  pub DeliveryTimeZone: String,
  pub DeliveryWindows: Vec<DeliveryWindow>,
}

impl<S: decode::XmlEventStream> decode::FromXMLStream<S> for ScheduledDeliveryInfo {
  fn from_xml(s: &mut S) -> decode::Result<ScheduledDeliveryInfo> {
    use xmlhelper::decode::{characters, fold_elements};
    fold_elements(s, ScheduledDeliveryInfo::default(), |s, record| {
      match s.local_name() {
        "DeliveryTimeZone" => record.DeliveryTimeZone = characters(s)?,
        "DeliveryWindows" => {
          record.DeliveryWindows = fold_elements(s, vec![], |s, v| {
            v.push(DeliveryWindow::from_xml(s)?);
            Ok(())
          })?
        }
        _ => {}
      }
      Ok(())
    })
  }
}

/// Information about a fulfillment order preview,
/// including delivery and fee information based on shipping method.
#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq)]
pub struct FulfillmentPreview {
  /// The shipping method for your fulfillment order.
  pub ShippingSpeedCategory: String,
  /// Indicates whether this fulfillment order preview is fulfillable.
  pub IsFulfillable: bool,
  /// Indicates whether this fulfillment order preview is for COD (Cash On Delivery).
  pub IsCODCapable: bool,
  /// The marketplace the fulfillment order is placed against.
  pub MarketplaceId: String,
  /// Estimated shipping weight for this fulfillment order preview.
  pub EstimatedShippingWeight: Option<Weight>,
  /// The estimated fulfillment fees for this fulfillment order
  /// preview, if applicable.
  pub EstimatedFees: Option<Vec<Fee>>,
  /// A list of fulfillable outbound shipments
  /// for this fulfillment order preview.
  pub FulfillmentPreviewShipments: Option<Vec<FulfillmentPreviewShipment>>,
  /// A list of unfulfillable items for this fulfillment order preview.
  pub UnfulfillablePreviewItems: Option<Vec<UnfulfillablePreviewItem>>,
  /// Error codes associated with the fulfillment order preview that indicate why the order is not fulfillable.
  pub OrderUnfulfillableReasons: Option<Vec<String>>,
  /// Delivery information for a Scheduled Delivery.
  pub ScheduledDeliveryInfo: Option<ScheduledDeliveryInfo>,
}

impl<S: decode::XmlEventStream> decode::FromXMLStream<S> for FulfillmentPreview {
  fn from_xml(s: &mut S) -> decode::Result<FulfillmentPreview> {
    use xmlhelper::decode::{characters, fold_elements};
    fold_elements(s, FulfillmentPreview::default(), |s, record| {
      match s.local_name() {
        "ShippingSpeedCategory" => record.ShippingSpeedCategory = characters(s)?,
        "IsFulfillable" => record.IsFulfillable = characters(s)?,
        "IsCODCapable" => record.IsCODCapable = characters(s)?,
        "MarketplaceId" => record.MarketplaceId = characters(s)?,
        "EstimatedShippingWeight" => {
          record.EstimatedShippingWeight = Weight::from_xml(s).map(Some)?
        }
        "EstimatedFees" => {
          record.EstimatedFees = fold_elements(s, vec![], |s, v| {
            v.push(Fee::from_xml(s)?);
            Ok(())
          }).map(Some)?
        }
        "FulfillmentPreviewShipments" => {
          record.FulfillmentPreviewShipments = fold_elements(s, vec![], |s, v| {
            v.push(FulfillmentPreviewShipment::from_xml(s)?);
            Ok(())
          }).map(Some)?
        }
        "UnfulfillablePreviewItems" => {
          record.UnfulfillablePreviewItems = fold_elements(s, vec![], |s, v| {
            v.push(UnfulfillablePreviewItem::from_xml(s)?);
            Ok(())
          }).map(Some)?
        }
        "OrderUnfulfillableReasons" => {
          record.OrderUnfulfillableReasons = fold_elements(s, vec![], |s, v| {
            v.push(characters(s)?);
            Ok(())
          }).map(Some)?
        }
        "ScheduledDeliveryInfo" => {
          record.ScheduledDeliveryInfo = ScheduledDeliveryInfo::from_xml(s).map(Some)?
        }
        _ => {}
      }
      Ok(())
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use chrono::TimeZone;
  use std::io::Cursor;
  use xmlhelper::decode;
  use xmlhelper::decode::FromXMLStream;

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

  #[test]
  fn test_package_tracking_details() {
    test_decode!(
      PackageTrackingDetails,
      r#"
        <ShipToAddress/>
        <AdditionalLocationInfo/>
        <CurrentStatus>IN_TRANSIT</CurrentStatus>
        <SignedForBy/>
        <ShipDate>2018-01-30T23:38:45Z</ShipDate>
        <PackageNumber>7777777777</PackageNumber>
        <CarrierCode>USPS</CarrierCode>
        <EstimatedArrivalDate>2018-02-01T08:00:00Z</EstimatedArrivalDate>
        <TrackingEvents>
          <member>
            <EventCode>EVENT_201</EventCode>
            <EventAddress>
              <Country>US</Country>
              <State>CALIFORNIA</State>
              <City>Newark</City>
            </EventAddress>
            <EventDate>2018-01-31T17:20:58Z</EventDate>
          </member>
          <member>
            <EventCode>EVENT_101</EventCode>
            <EventDate>2018-01-30T23:38:45Z</EventDate>
          </member>
        </TrackingEvents>
        <TrackingNumber>1818181818181818188181</TrackingNumber>
      "#,
      PackageTrackingDetails {
        PackageNumber: "7777777777".to_owned(),
        TrackingNumber: Some("1818181818181818188181".to_owned()),
        CarrierCode: Some("USPS".to_owned()),
        CarrierPhoneNumber: None,
        CarrierURL: None,
        ShipDate: Some(Utc.ymd(2018, 1, 30).and_hms(23, 38, 45)),
        ShipToAddress: Some(Default::default()),
        CurrentStatus: PackageStatus::IN_TRANSIT,
        SignedForBy: Some(String::default()),
        EstimatedArrivalDate: Some(Utc.ymd(2018, 2, 1).and_hms(8, 0, 0)),
        TrackingEvents: vec![
          TrackingEvent {
            EventCode: "EVENT_201".to_owned(),
            EventDate: Some(Utc.ymd(2018, 1, 31).and_hms(17, 20, 58)),
            EventAddress: Some(TrackingAddress {
              Country: "US".to_owned(),
              State: "CALIFORNIA".to_owned(),
              City: "Newark".to_owned(),
            }),
          },
          TrackingEvent {
            EventCode: "EVENT_101".to_owned(),
            EventDate: Some(Utc.ymd(2018, 1, 30).and_hms(23, 38, 45)),
            EventAddress: None,
          },
        ],
        AdditionalLocationInfo: Some(String::default()),
      }
    );
  }

  #[test]
  fn test_fulfillment_preview() {
    use chrono::Utc;
    test_decode!(
      FulfillmentPreview,
      r#"
        <MarketplaceId>A2EUQ1WTGCTBG2</MarketplaceId>
        <IsCODCapable>false</IsCODCapable>
        <ShippingSpeedCategory>Standard</ShippingSpeedCategory>
        <IsFulfillable>false</IsFulfillable>
        <UnfulfillablePreviewItems>
          <member>
            <SellerFulfillmentOrderItemId>2</SellerFulfillmentOrderItemId>
            <Quantity>139</Quantity>
            <SellerSKU>p2</SellerSKU>
            <ItemUnfulfillableReasons>
              <member>InventoryUnavailable</member>
            </ItemUnfulfillableReasons>
          </member>
        </UnfulfillablePreviewItems>
        <EstimatedFees>
          <member>
            <Name>FBAPerUnitFulfillmentFee</Name>
            <Amount>
              <CurrencyCode>CAD</CurrencyCode>
              <Value>441.00</Value>
            </Amount>
          </member>
          <member>
            <Name>FBATransportationFee</Name>
            <Amount>
              <CurrencyCode>CAD</CurrencyCode>
              <Value>16.34</Value>
            </Amount>
          </member>
        </EstimatedFees>
        <FulfillmentPreviewShipments>
          <member>
            <LatestShipDate>2018-05-18T17:08:05.000Z</LatestShipDate>
            <LatestArrivalDate>2018-05-25T17:08:05.000Z</LatestArrivalDate>
            <FulfillmentPreviewItems>
              <member>
                <SellerFulfillmentOrderItemId>1</SellerFulfillmentOrderItemId>
                <ShippingWeightCalculationMethod>Package</ShippingWeightCalculationMethod>
                <EstimatedShippingWeight>
                  <Value>5.420</Value>
                  <Unit>KILOGRAMS</Unit>
                </EstimatedShippingWeight>
                <Quantity>1</Quantity>
                <SellerSKU>p1</SellerSKU>
              </member>
            </FulfillmentPreviewItems>
            <EarliestArrivalDate>2018-05-23T17:08:05.000Z</EarliestArrivalDate>
            <EarliestShipDate>2018-05-18T17:08:05.000Z</EarliestShipDate>
          </member>
        </FulfillmentPreviewShipments>
      "#,
      FulfillmentPreview {
        ShippingSpeedCategory: "Standard".to_owned(),
        IsFulfillable: false,
        IsCODCapable: false,
        MarketplaceId: "A2EUQ1WTGCTBG2".to_owned(),
        EstimatedShippingWeight: None,
        EstimatedFees: Some(vec![
          Fee {
            Name: "FBAPerUnitFulfillmentFee".to_owned(),
            Amount: Currency {
              CurrencyCode: "CAD".to_owned(),
              Value: "441.00".to_owned(),
            },
          },
          Fee {
            Name: "FBATransportationFee".to_owned(),
            Amount: Currency {
              CurrencyCode: "CAD".to_owned(),
              Value: "16.34".to_owned(),
            },
          },
        ]),
        FulfillmentPreviewShipments: Some(vec![FulfillmentPreviewShipment {
          EarliestShipDate: Some(Utc.ymd(2018, 05, 18).and_hms(17, 8, 5)),
          LatestShipDate: Some(Utc.ymd(2018, 05, 18).and_hms(17, 8, 5)),
          EarliestArrivalDate: Some(Utc.ymd(2018, 05, 23).and_hms(17, 8, 5)),
          LatestArrivalDate: Some(Utc.ymd(2018, 05, 25).and_hms(17, 8, 5)),
          FulfillmentPreviewItems: vec![FulfillmentPreviewItem {
            SellerFulfillmentOrderItemId: "1".to_owned(),
            ShippingWeightCalculationMethod: Some("Package".to_owned()),
            EstimatedShippingWeight: Some(Weight {
              Unit: "KILOGRAMS".to_owned(),
              Value: "5.420".to_owned(),
            }),
            Quantity: 1,
            SellerSKU: "p1".to_owned(),
          }],
        }]),
        UnfulfillablePreviewItems: Some(vec![UnfulfillablePreviewItem {
          SellerSKU: "p2".to_owned(),
          SellerFulfillmentOrderItemId: "2".to_owned(),
          Quantity: 139,
          ItemUnfulfillableReasons: Some(vec!["InventoryUnavailable".to_owned()]),
        }]),
        OrderUnfulfillableReasons: None,
        ScheduledDeliveryInfo: None,
      }
    );
  }
}
