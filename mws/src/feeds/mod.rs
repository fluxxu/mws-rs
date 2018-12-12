//! Amazon MWS Feeds API - Version 2010-10-01
//!
//! [Documentation](http://docs.developer.amazonservices.com/en_CA/feeds/Feeds_Overview.html)

use chrono::{DateTime, Utc};
use client::{Client, ContentType, Method};
use result::MwsResult;
use std::io::{Read, Write};
use xmlhelper::encode;

mod inventory;
mod order_fulfillment_data;

static PATH: &'static str = "/";
static VERSION: &'static str = "2009-01-01";

/// Enumerates all the feed types that are available through the Feeds API section.
string_map_enum! {
  pub enum FeedType {
    Product = "_POST_PRODUCT_DATA_",
    Inventory = "_POST_INVENTORY_AVAILABILITY_DATA_",
    Overrides = "_POST_PRODUCT_OVERRIDES_DATA_",
    Pricing = "_POST_PRODUCT_PRICING_DATA_",
    ProductImages = "_POST_PRODUCT_IMAGE_DATA_",
    Relationships = "_POST_PRODUCT_RELATIONSHIP_DATA_",
    FlatFileInventoryLoader = "_POST_FLAT_FILE_INVLOADER_DATA_",
    FlatFileListings = "_POST_FLAT_FILE_LISTINGS_DATA_",
    FlatFileBookLoader = "_POST_FLAT_FILE_BOOKLOADER_DATA_",
    FlatFileMusicLoader = "_POST_FLAT_FILE_CONVERGENCE_LISTINGS_DATA_",
    // FlatFileVideoLoader = "_POST_FLAT_FILE_LISTINGS_DATA_",
    FlatFilePriceAndQuantityUpdate = "_POST_FLAT_FILE_PRICEANDQUANTITYONLY_UPDATE_DATA_",
    UIEEInventory = "_POST_UIEE_BOOKLOADER_DATA_",
    AutomotivePartFinder = "_POST_STD_ACES_DATA_",
  }
}

/// The optional OperationType element can be used to specify the type of operation (Update, Delete
/// or PartialUpdate) to be performed on the data. The OperationType is only applicable to productrelated
/// feeds (Product, Inventory, Price, etc) and will be ignored for non-applicable feeds.
#[derive(Debug, PartialEq, Serialize)]
pub enum OperationType {
  /// All specified information overwrites any existing information. Any
  /// unspecified information is erased.
  Update,

  /// All information is removed
  Delete,

  /// For Product feeds only: If you use PartialUpdate for a Product feed, all specified
  /// information overwrites any existing information, but unspecified information is
  /// unaffected. Caution: This operation type is only valid for Product feeds. If this operation
  /// type is used for any other feed type, such as Inventory and Price feeds, unpredictable
  /// data loss can occur.
  PartialUpdate,
}

pub trait Message {
  fn get_message_type() -> &'static str;
}

#[allow(non_snake_case)]
#[derive(Debug, PartialEq, Serialize)]
pub struct EnvelopeMessage<M: Message> {
  pub data: M,
  pub operation_type: Option<OperationType>,
}

#[allow(non_snake_case)]
#[derive(Debug, PartialEq, Serialize)]
pub struct Envelope<M: Message> {
  pub MerchantIdentifier: String,
  messages: Vec<EnvelopeMessage<M>>,
}

impl<M: Message> Envelope<M> {
  pub fn new(merchant_identifier: String) -> Self {
    Envelope::<M> {
      MerchantIdentifier: merchant_identifier,
      messages: vec![],
    }
  }

  pub fn add_message(&mut self, m: M, operation_type: Option<OperationType>) -> &mut Self {
    self.messages.push(EnvelopeMessage::<M> {
      data: m,
      operation_type: operation_type,
    });
    self
  }

  pub fn write_envelope_xml<W: encode::XmlEventWriter, F: FnMut(&mut W) -> encode::Result<()>>(
    &self,
    w: &mut W,
    mut f: F,
  ) -> encode::Result<()> {
    w.write(
      encode::XmlEvent::start_element("AmazonEnvelope")
        .ns("xsi", "http://www.w3.org/2001/XMLSchema-instance")
        .attr("xsi:noNamespaceSchemaLocation", "amznenvelope.xsd")
        .into(),
    )?;

    let mid: &str = self.MerchantIdentifier.as_ref();
    let message_type: &str = M::get_message_type();

    write_xml!(w,
      Header[][
        DocumentVersion[]["1.01"]
        MerchantIdentifier[][mid]
      ]
      MessageType[][message_type]
      [{ f(w) }]
    )?;

    w.write(encode::XmlEvent::end_element().into())
  }
}

#[allow(non_snake_case)]
#[derive(Debug, Default, SerializeMwsParams)]
pub struct SubmitFeedParameters {
  pub FeedType: String,

  pub MarketplaceIdList: Option<Vec<String>>,
  pub PurgeAndReplace: Option<bool>,
}

#[allow(non_snake_case)]
#[derive(Debug, Default, Serialize, FromXmlStream)]
pub struct SubmitFeedResponse {
  pub FeedSubmissionInfo: FeedSubmissionInfo,
}

response_envelope_type!(
  SubmitFeedEnvelope<SubmitFeedResponse>,
  "SubmitFeedResponse",
  "SubmitFeedResult"
);

#[allow(non_snake_case)]
pub fn SubmitFeed<R>(
  client: &Client,
  parameters: SubmitFeedParameters,
  content: R,
  content_md5: String,
  content_type: String,
) -> MwsResult<SubmitFeedResponse>
where
  R: Read + Send + 'static,
{
  client
    .request_xml_with_body(
      Method::Post,
      PATH,
      VERSION,
      "SubmitFeed",
      parameters,
      content,
      content_md5,
      ContentType(content_type.parse().unwrap()),
    )
    .map(|e: SubmitFeedEnvelope| e.into_inner())
    .map_err(Into::into)
}

#[allow(non_snake_case)]
pub fn GetFeedSubmissionResult<W: Write>(
  client: &Client,
  FeedSubmissionId: String,
  out: &mut W,
) -> MwsResult<u64> {
  let params = vec![("FeedSubmissionId".to_string(), FeedSubmissionId)];
  let mut resp = client.request(
    Method::Post,
    PATH,
    VERSION,
    "GetFeedSubmissionResult",
    params,
  )?;
  let size = ::std::io::copy(&mut resp, out)?;
  Ok(size)
}

/// Parameters for `GetFeedSubmissionList`
#[allow(non_snake_case)]
#[derive(Debug, Default, Serialize, SerializeMwsParams)]
pub struct GetFeedSubmissionListParameters {
  pub FeedSubmissionIdList: Option<Vec<String>>,
  pub MaxCount: Option<i32>,
  pub FeedTypeList: Option<Vec<String>>,
  pub FeedProcessingStatusList: Option<Vec<String>>,
  pub SubmittedFromDate: Option<DateTime<Utc>>,
  pub SubmittedToDate: Option<DateTime<Utc>>,
}

#[allow(non_snake_case)]
#[derive(Debug, Default, Serialize, FromXmlStream)]
pub struct FeedSubmissionInfo {
  pub FeedProcessingStatus: String,
  pub FeedType: String,
  pub FeedSubmissionId: String,
  pub StartedProcessingDate: Option<DateTime<Utc>>,
  pub SubmittedDate: Option<DateTime<Utc>>,
  pub CompletedProcessingDate: Option<DateTime<Utc>>,
}

#[allow(non_snake_case)]
#[derive(Debug, Default, Serialize, FromXmlStream)]
pub struct GetFeedSubmissionListResponse {
  pub RequestId: String,
  #[from_xml_stream(no_list_wrapper)]
  pub FeedSubmissionInfo: Vec<FeedSubmissionInfo>,
  pub NextToken: Option<String>,
}

response_envelope_type!(
  GetFeedSubmissionListEnvelope<GetFeedSubmissionListResponse>,
  "GetFeedSubmissionListResponse",
  "GetFeedSubmissionListResult"
);

response_envelope_type!(
  GetFeedSubmissionListByNextTokenEnvelope<GetFeedSubmissionListResponse>,
  "GetFeedSubmissionListByNextTokenResponse",
  "GetFeedSubmissionListByNextTokenResult"
);

#[allow(non_snake_case)]
pub fn GetFeedSubmissionList(
  client: &Client,
  parameters: GetFeedSubmissionListParameters,
) -> MwsResult<GetFeedSubmissionListResponse> {
  client
    .request_xml(
      Method::Post,
      PATH,
      VERSION,
      "GetFeedSubmissionList",
      parameters,
    )
    .map(|e: GetFeedSubmissionListEnvelope| e.into_inner())
    .map_err(|err| err.into())
}

#[allow(non_snake_case)]
pub fn GetFeedSubmissionListByNextToken(
  client: &Client,
  next_token: String,
) -> MwsResult<GetFeedSubmissionListResponse> {
  let params = vec![("NextToken".to_string(), next_token)];
  client
    .request_xml(
      Method::Post,
      PATH,
      VERSION,
      "GetFeedSubmissionListByNextToken",
      params,
    )
    .map(|e: GetFeedSubmissionListByNextTokenEnvelope| e.into_inner())
    .map_err(|err| err.into())
}
