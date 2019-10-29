use chrono::{DateTime, Utc};

str_enum! {
  /// The FulfillmentPolicy value that you chose when
  /// you submitted the CreateFulfillmentOrder operation.
  pub enum FulfillmentPolicy {
    FillOrKill,
    FillAll,
    FillAllAvailable,
  }
}

str_enum! {
  /// The current status of the fulfillment order.
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
#[derive(Debug, Default, PartialEq, Serialize, FromXmlStream)]
pub struct FulfillmentOrder {
  /// The fulfillment order identifier that you created and
  /// submitted using the CreateFulfillmentOrder operation.
  pub SellerFulfillmentOrderId: String,
  /// The destination address that you created when you submitted
  /// the CreateFulfillmentOrder operation.
  pub DestinationAddress: DestinationAddress,
  /// A date that you created when you submitted the CreateFulfillmentOrder
  /// operation. Displays as the order date in recipient-facing materials such
  /// as the packing slip.
  pub DisplayableOrderDateTime: Option<DateTime<Utc>>,
  /// The shipping method that you selected when you submitted the
  /// CreateFulfillmentOrder operation.
  pub ShippingSpeedCategory: ShippingSpeedCategory,
  pub FulfillmentMethod: String,
  /// The current status of the fulfillment order.
  pub FulfillmentOrderStatus: FulfillmentOrderStatus,
  /// The date that the status of the fulfillment order last changed.
  pub StatusUpdatedDateTime: Option<DateTime<Utc>>,
  /// The FulfillmentPolicy value that you chose when you submitted
  /// the CreateFulfillmentOrder operation.
  pub FulfillmentPolicy: FulfillmentPolicy,
  /// The date that the fulfillment order was received by an Amazon
  /// fulfillment center.
  pub ReceivedDateTime: Option<DateTime<Utc>>,
  /// A fulfillment order identifier that you created when you submitted
  /// the CreateFulfillmentOrder operation. Displays as the order identifier
  /// in recipient-facing materials such as the packing slip.
  pub DisplayableOrderId: String,
  /// A text block that you created when you submitted the CreateFulfillmentOrder
  /// operation. Displays in recipient-facing materials such as the packing slip.
  pub DisplayableOrderComment: String,
  /// The marketplace the fulfillment order is placed against.
  pub MarketplaceId: Option<String>,
  /// Specifies whether an order was created to ship immediately or to be held
  /// for later. Only returned by GetFulfillmentOrder.
  pub FulfillmentAction: Option<String>,
  /// The NotificationEmailList value that you created when you submitted the
  /// CreateFulfillmentOrder operation.
  pub NotificationEmailList: Vec<String>,
}

#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq, Serialize, SerializeMwsParams, FromXmlStream)]
pub struct Currency {
  /// Three-digit currency code.
  pub CurrencyCode: String,
  /// The currency amount.
  pub Value: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Default, Serialize, FromXmlStream)]
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

#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq, Serialize, SerializeMwsParams, FromXmlStream)]
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

// #[derive(Debug, Default, Serialize)]
// pub struct ReturnItemList;

// #[derive(Debug, Default, Serialize)]
// pub struct ReturnAuthorizationList;

str_enum! {
  /// The current status of the shipment.
  pub enum FulfillmentShipmentStatus {
    PENDING,
    SHIPPED,
    CANCELLED_BY_FULFILLER,
    CANCELLED_BY_SELLER,
  }
}

/// Delivery and item information for a shipment in a fulfillment order.
#[allow(non_snake_case)]
#[derive(Debug, Default, Serialize, FromXmlStream)]
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

/// Item information for a shipment in a fulfillment order.
#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq, Serialize, FromXmlStream)]
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

#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq, Serialize, FromXmlStream)]
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

str_enum! {
  /// The current status of the fulfillment order.
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
#[derive(Debug, Default, PartialEq, Serialize, FromXmlStream)]
pub struct TrackingAddress {
  pub City: String,
  pub State: String,
  pub Country: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq, Serialize, FromXmlStream)]
pub struct TrackingEvent {
  pub EventDate: Option<DateTime<Utc>>,
  pub EventAddress: Option<TrackingAddress>,
  pub EventCode: String,
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
#[derive(Debug, Default, PartialEq, Serialize, FromXmlStream)]
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

str_enum! {
  /// The shipping method for your fulfillment order.
  pub enum ShippingSpeedCategory {
    Standard,
    Expedited,
    Priority,
    ScheduledDelivery,
  }
}

str_enum! {
  /// Specifies whether the fulfillment order should
  /// ship now or have an order hold put on it.
  pub enum FulfillmentAction {
    Ship,
    Hold,
  }
}

/// Weight unit and amount.
#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq, Serialize, FromXmlStream)]
pub struct Weight {
  /// Indicates the unit of weight.
  pub Unit: String,
  /// The numeric value of the item's weight.
  pub Value: String,
}

/// Fee type and cost.
#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq, Serialize, FromXmlStream)]
pub struct Fee {
  /// The type of fee.
  pub Name: String,
  /// The numeric value of the item's weight.
  pub Amount: Currency,
}

/// Item information for a shipment in a fulfillment order preview.
#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq, Serialize, FromXmlStream)]
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

/// Delivery and item information for a shipment in a fulfillment order preview.
#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq, Serialize, FromXmlStream)]
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

/// Information about unfulfillable items in a fulfillment order preview.
#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq, Serialize, FromXmlStream)]
pub struct UnfulfillablePreviewItem {
  pub SellerSKU: String,
  pub SellerFulfillmentOrderItemId: String,
  pub Quantity: i32,
  /// Error codes associated with the fulfillment order
  /// preview that indicate why the item is unfulfillable.
  pub ItemUnfulfillableReasons: Option<Vec<String>>,
}

/// The time range within which your Scheduled Delivery fulfillment order should be delivered.
#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq, Serialize, FromXmlStream)]
pub struct DeliveryWindow {
  pub StartDateTime: Option<DateTime<Utc>>,
  pub EndDateTime: Option<DateTime<Utc>>,
}

/// Delivery information for a Scheduled Delivery.
#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq, Serialize, FromXmlStream)]
pub struct ScheduledDeliveryInfo {
  pub DeliveryTimeZone: String,
  pub DeliveryWindows: Vec<DeliveryWindow>,
}

/// Information about a fulfillment order preview,
/// including delivery and fee information based on shipping method.
#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq, Serialize, FromXmlStream)]
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

#[cfg(test)]
mod tests {
  use super::*;
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
        ShippingSpeedCategory: ShippingSpeedCategory::Expedited,
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
