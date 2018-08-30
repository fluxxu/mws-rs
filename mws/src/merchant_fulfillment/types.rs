use chrono::{DateTime, Utc};
use result::MwsResult;
use xmlhelper::decode;

#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq, Serialize, SerializeMwsParams)]
pub struct ShipmentRequestDetails {
  pub AmazonOrderId: String,
  pub SellerOrderId: Option<String>,
  pub ItemList: Vec<Item>,
  pub ShipFromAddress: Address,
  pub PackageDimensions: PackageDimensions,
  pub Weight: Weight,
  pub Insurance: Option<CurrencyAmount>,
  pub MustArriveByDate: Option<DateTime<Utc>>,
  pub ShipDate: Option<DateTime<Utc>>,
  pub ShippingServiceOptions: ShippingServiceOptions,
  pub LabelCustomization: LabelCustomization,
}

#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq, Serialize, SerializeMwsParams, FromXmlStream)]
pub struct Item {
  /// An Amazon-defined identifier for an individual
  /// item in an order. Used in the XML response to
  /// an order query request (Order API/Order XML).
  pub OrderItemId: String,

  /// The number of items.
  pub Quantity: i32,
}

#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq, Serialize, SerializeMwsParams, FromXmlStream)]
pub struct Address {
  pub Name: String,
  pub AddressLine1: String,
  pub AddressLine2: String,
  pub AddressLine3: String,
  pub DistrictOrCounty: String,
  pub Email: String,
  pub City: String,
  pub StateOrProvinceCode: String,
  pub PostalCode: String,
  pub CountryCode: String,
  pub Phone: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq, Serialize, SerializeMwsParams, FromXmlStream)]
pub struct PackageDimensions {
  pub Length: String,
  pub Width: String,
  pub Height: String,
  pub Unit: DimensionsUnit,
  pub PredefinedPackageDimensions: Option<String>,
}

str_enum! {
  pub enum DimensionsUnit {
    centimeters,
    inches,
  }
}

#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq, Serialize, SerializeMwsParams, FromXmlStream)]
pub struct Weight {
  pub Value: String,
  pub Unit: WeightUnit,
}

str_enum! {
  pub enum WeightUnit {
    g,
    ounces,
    oz,
    grams,
  }
}

str_enum! {
  pub enum DeliveryExperience {
    DeliveryConfirmationWithAdultSignature,
    DeliveryConfirmationWithSignature,
    DeliveryConfirmationWithoutSignature,
    NoTracking,
  }
}

#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq, Serialize, SerializeMwsParams, FromXmlStream)]
pub struct CurrencyAmount {
  CurrencyCode: String,
  Amount: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq, Serialize, SerializeMwsParams)]
pub struct ShippingServiceOptions {
  pub DeliveryExperience: DeliveryExperience,
  pub DeclaredValue: Option<CurrencyAmount>,
  pub CarrierWillPickUp: bool,
  pub LabelFormat: Option<String>,
}

impl<S: decode::XmlEventStream> decode::FromXmlStream<S> for ShippingServiceOptions {
  fn from_xml(s: &mut S) -> MwsResult<ShippingServiceOptions> {
    use xmlhelper::decode::{characters, fold_elements};
    fold_elements(s, ShippingServiceOptions::default(), |s, record| {
      match s.local_name() {
        "DeliveryExperience" => record.DeliveryExperience = characters(s)?,
        "CarrierWillPickUp" => record.CarrierWillPickUp = characters(s)?,
        _ => {}
      }
      Ok(())
    })
  }
}

#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq, Serialize, SerializeMwsParams)]
pub struct LabelCustomization {
  pub CustomTextForLabel: Option<String>,
  pub StandardIdForLabel: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq, Serialize, FromXmlStream)]
pub struct ShippingService {
  pub ShippingServiceName: String,
  pub CarrierName: String,
  pub ShippingServiceId: String,
  pub ShippingServiceOfferId: String,
  pub ShipDate: Option<DateTime<Utc>>,
  pub EarliestEstimatedDeliveryDate: Option<DateTime<Utc>>,
  pub LatestEstimatedDeliveryDate: Option<DateTime<Utc>>,
  pub Rate: CurrencyAmount,
  pub ShippingServiceOptions: ShippingServiceOptions,
  /// May include PNG, PDF, and ZPL203.
  pub AvailableLabelFormats: Vec<String>,
}

#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq, Serialize, FromXmlStream)]
pub struct Carrier {
  pub CarrierName: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq, Serialize, FromXmlStream)]
pub struct Shipment {
  pub ShipmentId: String,
  pub AmazonOrderId: String,
  pub SellerOrderId: Option<String>,
  pub ItemList: Vec<Item>,
  pub ShipFromAddress: Address,
  pub ShipToAddress: Address,
  pub PackageDimensions: PackageDimensions,
  pub Weight: Weight,
  pub Insurance: Option<CurrencyAmount>,
  pub ShippingService: ShippingService,
  pub Label: Label,
  pub Status: ShipmentStatus,
  pub TrackingId: Option<String>,
  pub CreatedDate: Option<DateTime<Utc>>,
  pub LastUpdatedDate: Option<DateTime<Utc>>,
}

str_enum! {
  pub enum ShipmentStatus {
    Purchased,
    RefundPending,
    RefundRejected,
    RefundApplied,
  }
}

#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq, Serialize, FromXmlStream)]
pub struct Label {
  pub CustomTextForLabel: Option<String>,
  pub Dimensions: LabelDimensions,
  pub FileContents: FileContents,
  pub LabelFormat: Option<String>,
  pub StandardIdForLabel: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq, Serialize, FromXmlStream)]
pub struct LabelDimensions {
  pub Length: String,
  pub Width: String,
  pub Unit: DimensionsUnit,
}

#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq, Serialize, FromXmlStream)]
pub struct FileContents {
  pub Contents: String,
  pub FileType: String,
  pub Checksum: String,
}

str_enum! {
  pub enum HazmatType {
    None,
    LQHazmat,
  }
}
